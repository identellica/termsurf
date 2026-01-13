use super::{stream_resource_handler::StreamResourceHandler, zip_archive::*};
use crate::*;
use std::{
    collections::{BTreeMap, VecDeque},
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};

pub type UrlFilter = Box<dyn Send + Sync + Fn(&str) -> String>;
pub type MimeTypeResolver = Box<dyn Send + Sync + Fn(&str) -> String>;

struct RequestParams {
    url: String,
    browser: Browser,
    frame: Frame,
    request: Request,
    url_filter: Arc<UrlFilter>,
    mime_type_resolver: Arc<MimeTypeResolver>,
}

#[derive(Default)]
struct ProviderEntry {
    provider: Option<Box<dyn ResourceManagerProvider>>,
    order: i32,
    identifier: String,
    pending_requests: VecDeque<Arc<Mutex<ResourceManagerRequest>>>,
    deletion_pending: bool,
}

/// Values associated with the pending request only. Ownership will be passed
/// between requests and the resource manager as request handling proceeds.
struct RequestState {
    manager: Weak<Mutex<ResourceManager>>,

    /// Callback to execute once request handling is complete.
    callback: Option<Callback>,

    /// Position of the currently associated [ProviderEntry] in the `ResourceManagerProviders`
    /// list.
    current_entry: usize,

    /// Position of this request object in the currently associated
    /// [ProviderEntry]'s `pending_requests` list.
    current_request: usize,

    /// Params that will be copied to each request object.
    params: Arc<RequestParams>,
}

impl Drop for RequestState {
    fn drop(&mut self) {
        // Always execute the callback.
        if let Some(callback) = self.callback.take() {
            callback.cont();
        }
    }
}

pub struct ResourceManagerRequest {
    weak_self: Weak<Mutex<Self>>,
    state: Option<RequestState>,
    params: Arc<RequestParams>,
}

impl ResourceManagerRequest {
    fn new(state: Option<RequestState>, params: Arc<RequestParams>) -> Arc<Mutex<Self>> {
        Arc::new_cyclic(|weak_self| {
            Mutex::new(Self {
                weak_self: weak_self.clone(),
                state,
                params,
            })
        })
    }

    /// Returns the URL associated with this request. The returned value will be fully qualified
    /// but will not contain query or fragment components. It will already have been passed through
    /// the URL filter.
    pub fn url(&self) -> &str {
        &self.params.url
    }

    /// Returns the [Browser] associated with this request.
    pub fn browser(&self) -> &Browser {
        &self.params.browser
    }

    /// Returns the [Frame] associated with this request.
    pub fn frame(&self) -> &Frame {
        &self.params.frame
    }

    /// Returns the [Request] associated with this request.
    pub fn request(&self) -> &Request {
        &self.params.request
    }

    /// Returns the current URL filter.
    pub fn url_filter(&self) -> &UrlFilter {
        &self.params.url_filter
    }

    /// Returns the current mime type resolver.
    pub fn mime_type_resolver(&self) -> &MimeTypeResolver {
        &self.params.mime_type_resolver
    }

    pub fn continue_request(&mut self, handler: Option<ResourceHandler>) {
        // Disassociate `self.state` immediately so that [ResourceManagerProvider::on_request_canceled]
        // is not called unexpectedly if [ResourceManagerProvider::on_request] calls this method
        // and then calls [ResourceManager::remove].
        let Some(state) = self.state.take() else {
            return;
        };

        let io_thread_id = ThreadId::IO;
        let mut task = ContinueRequest::new(Arc::new(Mutex::new(Some(state))), handler);
        post_task(io_thread_id, Some(&mut task));
    }

    pub fn stop_request(&mut self) {
        // Disassociate `self.state` immediately so that [ResourceManagerProvider::on_request_canceled]
        // is not called unexpectedly if [ResourceManagerProvider::on_request] calls this method
        // and then calls [ResourceManager::remove].
        let Some(state) = self.state.take() else {
            return;
        };

        let io_thread_id = ThreadId::IO;
        let mut task = StopRequest::new(Arc::new(Mutex::new(Some(state))));
        post_task(io_thread_id, Some(&mut task));
    }

    /// Detaches and returns `self.state` if the provider indicates that it will not handle the
    /// request. Note that `self.state` may already be [None] if [ResourceManagerProvider::on_request]
    /// executes a callback before returning, in which case execution will continue asynchronously
    /// in any case.
    fn send_request(&mut self) -> Option<RequestState> {
        debug_assert_ne!(
            currently_on(ThreadId::IO),
            0,
            "send_request must be called on the IO thread"
        );

        let state = self.state.as_ref()?;
        let Some(manager) = state.manager.upgrade() else {
            return self.state.take();
        };
        let Ok(mut manager) = manager.lock() else {
            return self.state.take();
        };
        let Some(provider_entry) = manager.providers.get_mut(state.current_entry) else {
            return self.state.take();
        };
        let Some(provider) = provider_entry.provider.as_mut() else {
            return self.state.take();
        };

        let Some(request) = self.weak_self.upgrade() else {
            return self.state.take();
        };
        if !provider.on_request(request) {
            return self.state.take();
        }

        None
    }
}

wrap_task! {
    struct ContinueRequest {
        state: Arc<Mutex<Option<RequestState>>>,
        handler: Option<ResourceHandler>,
    }

    impl Task {
        fn execute(&self) {
            let Ok(mut state) = self.state.lock() else {
                return;
            };
            let Some(state) = state.take() else {
                return;
            };
            let Some(manager) = state.manager.upgrade() else {
                return;
            };
            let Ok(mut manager) = manager.lock() else {
                return;
            };

            manager.continue_request(state, self.handler.clone());
        }
    }
}

wrap_task! {
    struct StopRequest {
        state: Arc<Mutex<Option<RequestState>>>,
    }

    impl Task {
        fn execute(&self) {
            let Ok(mut state) = self.state.lock() else {
                return;
            };
            let Some(state) = state.take() else {
                return;
            };
            let Some(manager) = state.manager.upgrade() else {
                return;
            };
            let Ok(mut manager) = manager.lock() else {
                return;
            };

            manager.stop_request(state);
        }
    }
}

/// Interface implemented by resource [ResourceManagerProviders]. A [ResourceManagerProvider] may
/// be created on any thread but the methods will be called on, and the object will be destroyed
/// on, the browser process IO thread.
pub trait ResourceManagerProvider: Send {
    /// Called to handle a request. If the ResourceManagerProvider knows immediately that it will
    /// not handle the request return false. Otherwise, return true and call [ResourceManagerRequest::continue_request]
    /// or [ResourceManagerRequest::stop_request] either in this method or asynchronously to
    /// indicate completion. See comments on [ResourceManagerRequest] for additional usage
    /// information.
    fn on_request(&self, request: Arc<Mutex<ResourceManagerRequest>>) -> bool;

    /// Called when a request has been canceled. It is still safe to dereference `request` but any
    /// calls to [ResourceManagerRequest::continue_request] or [ResourceManagerRequest::stop_request]
    /// will be ignored.
    fn on_request_canceled(&self, _request: Arc<Mutex<ResourceManagerRequest>>) {}
}

/// Provider of fixed contents.
struct ContentProvider {
    url: String,
    content: String,
    mime_type: String,
}

impl ContentProvider {
    fn new_resource_manager_provider(
        url: &str,
        content: &str,
        mime_type: &str,
    ) -> Box<dyn ResourceManagerProvider> {
        Box::new(Self {
            url: url.to_string(),
            content: content.to_string(),
            mime_type: mime_type.to_string(),
        })
    }
}

impl ResourceManagerProvider for ContentProvider {
    fn on_request(&self, request: Arc<Mutex<ResourceManagerRequest>>) -> bool {
        debug_assert_ne!(
            currently_on(ThreadId::IO),
            0,
            "on_request must be called on the IO thread"
        );

        let Ok(mut request) = request.lock() else {
            return false;
        };

        if request.url() != self.url {
            // Not handled by this provider.
            return false;
        }

        let mut data: Vec<_> = self.content.bytes().collect();
        let Some(stream) = stream_reader_create_for_data(data.as_mut_ptr(), data.len()) else {
            return false;
        };

        let mime_type = if self.mime_type.is_empty() {
            self.mime_type.clone()
        } else {
            (request.mime_type_resolver())(&self.url)
        };

        request.continue_request(Some(StreamResourceHandler::new_with_stream(
            mime_type, stream,
        )));
        true
    }
}

/// Provider of contents loaded from a directory on the file system.
struct DirectoryProvider {
    url_path: String,
    directory_path: PathBuf,
}

impl DirectoryProvider {
    fn new_resource_manager_provider(
        url_path: &str,
        directory_path: PathBuf,
    ) -> Box<dyn ResourceManagerProvider> {
        Box::new(Self {
            url_path: normalize_url_path(url_path),
            directory_path,
        })
    }
}

impl ResourceManagerProvider for DirectoryProvider {
    fn on_request(&self, request: Arc<Mutex<ResourceManagerRequest>>) -> bool {
        debug_assert_ne!(
            currently_on(ThreadId::IO),
            0,
            "on_request must be called on the IO thread"
        );

        let Some(file_path) = ({
            let Ok(request) = request.lock() else {
                return false;
            };
            let url = request.url();
            if !url.starts_with(self.url_path.as_str()) {
                // Not handled by this provider.
                return false;
            }

            get_file_relative_path(self.directory_path.clone(), self.url_path.as_str(), url)
        }) else {
            return false;
        };

        let mut task = OpenFileRequest::new(file_path, request);
        post_task(ThreadId::FILE_USER_BLOCKING, Some(&mut task));

        true
    }
}

wrap_task! {
    struct OpenFileRequest {
        file_path: PathBuf,
        request: Arc<Mutex<ResourceManagerRequest>>,
    }

    impl Task {
        fn execute(&self) {
            debug_assert_ne!(
                currently_on(ThreadId::FILE_USER_BLOCKING),
                0,
                "execute must be called on the file thread"
            );

            let file_path = self.file_path.display().to_string();
            let Some(stream) =
                stream_reader_create_for_file(Some(&CefString::from(file_path.as_str())))
            else {
                return;
            };

            // Continue loading on the IO thread.
            let mut task = ContinueFileRequest::new(stream, self.request.clone());
            post_task(ThreadId::IO, Some(&mut task));
        }
    }
}

wrap_task! {
    struct ContinueFileRequest {
        stream: StreamReader,
        request: Arc<Mutex<ResourceManagerRequest>>,
    }

    impl Task {
        fn execute(&self) {
            debug_assert_ne!(
                currently_on(ThreadId::IO),
                0,
                "execute must be called on the IO thread"
            );

            let Ok(mut request) = self.request.lock() else {
                return;
            };

            let handler = StreamResourceHandler::new_with_stream(
                (request.mime_type_resolver())(request.url()),
                self.stream.clone(),
            );
            request.continue_request(Some(handler));
        }
    }
}

struct ArchiveProviderState {
    weak_self: Weak<Mutex<ArchiveProviderState>>,
    url_path: String,
    archive_path: PathBuf,
    password: String,
    load_started: bool,
    load_ended: bool,
    archive: Option<ZipArchive>,
    pending_requests: Vec<Arc<Mutex<ResourceManagerRequest>>>,
}

impl ArchiveProviderState {
    fn new(url_path: &str, archive_path: PathBuf, password: &str) -> Arc<Mutex<Self>> {
        Arc::new_cyclic(|weak_self| {
            Mutex::new(Self {
                weak_self: weak_self.clone(),
                url_path: normalize_url_path(url_path),
                archive_path,
                password: password.to_string(),
                load_started: false,
                load_ended: false,
                archive: None,
                pending_requests: Default::default(),
            })
        })
    }

    fn continue_request(&mut self, request: Arc<Mutex<ResourceManagerRequest>>) -> bool {
        debug_assert_ne!(
            currently_on(ThreadId::IO),
            0,
            "continue_request must be called on the IO thread"
        );

        let Ok(mut request) = request.lock() else {
            return false;
        };

        let url = request.url();
        let (Some(archive), Some(relative_path)) = (
            self.archive.as_ref(),
            get_file_relative_path(Default::default(), self.url_path.as_str(), url),
        ) else {
            return false;
        };

        let relative_path = relative_path.display().to_string();
        let Some(file) = archive.file(&relative_path) else {
            return false;
        };
        let Ok(file) = file.lock() else {
            return false;
        };
        let Some(stream) = file.stream_reader() else {
            return false;
        };

        let handler =
            StreamResourceHandler::new_with_stream((request.mime_type_resolver())(url), stream);
        request.continue_request(Some(handler));
        true
    }
}

/// Provider of contents loaded from an archive file.
struct ArchiveProvider {
    state: Arc<Mutex<ArchiveProviderState>>,
}

impl ArchiveProvider {
    fn new_resource_manager_provider(
        url_path: &str,
        archive_path: PathBuf,
        password: &str,
    ) -> Box<dyn ResourceManagerProvider> {
        Box::new(Self {
            state: ArchiveProviderState::new(url_path, archive_path, password),
        })
    }
}

impl ResourceManagerProvider for ArchiveProvider {
    fn on_request(&self, request: Arc<Mutex<ResourceManagerRequest>>) -> bool {
        debug_assert_ne!(
            currently_on(ThreadId::IO),
            0,
            "on_request must be called on the IO thread"
        );

        let (Ok(mut state), Ok(request_lock)) = (self.state.lock(), request.lock()) else {
            return false;
        };
        let url = request_lock.url();
        if !url.starts_with(state.url_path.as_str()) {
            // Not handled by this provider.
            return false;
        }

        if !state.load_started {
            // Initiate archive loading and queue the pending request.
            state.load_started = true;
            state.pending_requests.push(request.clone());

            // Load the archive file on the FILE thread.
            let mut task = OpenZipRequest::new(state.weak_self.clone());
            post_task(ThreadId::FILE_USER_BLOCKING, Some(&mut task));
            return true;
        }

        if state.load_started && !state.load_ended {
            // The archive load has already started. Queue the pending request.
            state.pending_requests.push(request.clone());
            return true;
        }

        // Archive loading is done.
        state.continue_request(request.clone())
    }
}

wrap_task! {
    struct OpenZipRequest {
        state: Weak<Mutex<ArchiveProviderState>>,
    }

    impl Task {
        fn execute(&self) {
            debug_assert_ne!(
                currently_on(ThreadId::FILE_USER_BLOCKING),
                0,
                "execute must be called on the file thread"
            );

            let Some(state) = self.state.upgrade() else {
                return;
            };
            let Ok(mut state) = state.lock() else {
                return;
            };

            let file_path = state.archive_path.display().to_string();
            let Some(mut stream) =
                stream_reader_create_for_file(Some(&CefString::from(file_path.as_str())))
            else {
                return;
            };

            let archive = ZipArchive::default();
            if archive.load(&mut stream, state.password.as_str(), true) == 0 {
                return;
            }
            state.archive = Some(archive);

            // Continue loading on the IO thread.
            let mut task = ContinueZipRequest::new(state.weak_self.clone());
            post_task(ThreadId::IO, Some(&mut task));
        }
    }
}

wrap_task! {
    struct ContinueZipRequest {
        state: Weak<Mutex<ArchiveProviderState>>,
    }

    impl Task {
        fn execute(&self) {
            debug_assert_ne!(
                currently_on(ThreadId::IO),
                0,
                "execute must be called on the IO thread"
            );

            let Some(state) = self.state.upgrade() else {
                return;
            };
            let Ok(mut state) = state.lock() else {
                return;
            };

            state.load_ended = true;

            for request in std::mem::take(&mut state.pending_requests) {
                state.continue_request(request);
            }
        }
    }
}

wrap_task! {
    struct AddProvider {
        manager: Weak<Mutex<ResourceManager>>,
        provider: Arc<Mutex<Option<Box<dyn ResourceManagerProvider>>>>,
        order: i32,
        identifier: String,
    }

    impl Task {
        fn execute(&self) {
            let Some(manager) = self.manager.upgrade() else {
                return;
            };
            let Ok(mut manager) = manager.lock() else {
                return;
            };
            let Ok(mut provider) = self.provider.lock() else {
                return;
            };
            let Some(provider) = provider.take() else {
                return;
            };

            manager.add_provider(provider, self.order, &self.identifier);
        }
    }
}

wrap_task! {
    struct RemoveProviders {
        manager: Weak<Mutex<ResourceManager>>,
        identifier: String,
    }

    impl Task {
        fn execute(&self) {
            let Some(manager) = self.manager.upgrade() else {
                return;
            };
            let Ok(mut manager) = manager.lock() else {
                return;
            };

            manager.remove_providers(&self.identifier);
        }
    }
}

wrap_task! {
    struct RemoveAllProviders {
        manager: Weak<Mutex<ResourceManager>>,
    }

    impl Task {
        fn execute(&self) {
            let Some(manager) = self.manager.upgrade() else {
                return;
            };
            let Ok(mut manager) = manager.lock() else {
                return;
            };

            manager.remove_all_providers();
        }
    }
}

wrap_task! {
    struct SetUrlFilter {
        manager: Weak<Mutex<ResourceManager>>,
        filter: Arc<Mutex<Option<Option<UrlFilter>>>>,
    }

    impl Task {
        fn execute(&self) {
            let Some(manager) = self.manager.upgrade() else {
                return;
            };
            let Ok(mut manager) = manager.lock() else {
                return;
            };
            let Ok(mut filter) = self.filter.lock() else {
                return;
            };
            let Some(filter) = filter.take() else {
                return;
            };

            manager.set_url_filter(filter);
        }
    }
}

wrap_task! {
    struct SetMimeTypeResolver {
        manager: Weak<Mutex<ResourceManager>>,
        resolver: Arc<Mutex<Option<Option<MimeTypeResolver>>>>,
    }

    impl Task {
        fn execute(&self) {
            let Some(manager) = self.manager.upgrade() else {
                return;
            };
            let Ok(mut manager) = manager.lock() else {
                return;
            };
            let Ok(mut resolver) = self.resolver.lock() else {
                return;
            };
            let Some(resolver) = resolver.take() else {
                return;
            };

            manager.set_mime_type_resolver(resolver);
        }
    }
}

/// Type for managing multiple resource providers. For each resource request, providers will be
/// called in order and have the option to:
/// - (a) handle the request by returning a [ResourceHandler],
/// - (b) pass the request to the next provider in order, or
/// - (c) stop handling the request.
///
/// See comments on the [ResourceManagerRequest] object for additional usage information. The
/// methods of this class may be called on any browser process thread unless otherwise indicated.
pub struct ResourceManager {
    providers: VecDeque<ProviderEntry>,
    pending_handlers: BTreeMap<u64, ResourceHandler>,
    mime_type_resolver: Arc<MimeTypeResolver>,
    url_filter: Arc<UrlFilter>,
    weak_self: Weak<Mutex<Self>>,
}

impl ResourceManager {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new_cyclic(|weak_self| {
            Mutex::new(Self {
                providers: Default::default(),
                pending_handlers: Default::default(),
                mime_type_resolver: Arc::new(Box::new(get_mime_type)),
                url_filter: Arc::new(Box::new(get_filtered_url)),
                weak_self: weak_self.clone(),
            })
        })
    }

    /// Add a provider that maps requests for `url` to `content`. `url` should be fully qualified
    /// but not include a query or fragment component. If `mime_type` is empty the [MimeTypeResolver]
    /// will be used. See comments on [ResourceManager::add_provider] for usage of the `order` and
    /// `identifier` parameters.
    pub fn add_content_provider(
        &mut self,
        url: &str,
        content: &str,
        mime_type: &str,
        order: i32,
        identifier: &str,
    ) {
        self.add_provider(
            ContentProvider::new_resource_manager_provider(url, content, mime_type),
            order,
            identifier,
        )
    }

    /// Add a provider that maps requests that start with `url_path` to files under `archive_path`.
    /// `url_path` should include an origin and optional path component only. Files will be loaded
    /// when a matching URL is requested. See comments on [ResourceManager::add_provider] for usage
    /// of the `order` and `identifier` parameters.
    pub fn add_directory_provider(
        &mut self,
        url_path: &str,
        directory_path: &str,
        order: i32,
        identifier: &str,
    ) {
        self.add_provider(
            DirectoryProvider::new_resource_manager_provider(url_path, directory_path.into()),
            order,
            identifier,
        )
    }

    /// Add a provider that maps requests that start with `url_path` to files stored in the archive
    /// file at `archive_path`. `url_path` should include an origin and optional path component
    /// only. The archive file will be loaded when a matching URL is requested for the first time.
    /// See comments on [ResourceManager::add_provider] for usage of the `order` and `identifier`
    /// parameters.
    pub fn add_archive_provider(
        &mut self,
        url_path: &str,
        archive_path: &str,
        password: &str,
        order: i32,
        identifier: &str,
    ) {
        self.add_provider(
            ArchiveProvider::new_resource_manager_provider(url_path, archive_path.into(), password),
            order,
            identifier,
        )
    }

    /// Add a provider. This object takes ownership of `provider`. Providers will be called in
    /// ascending order based on the `order` value. Multiple providers sharing the same `order`
    /// value will be called in the order that they were added. The `identifier` value, which does
    /// not need to be unique, can be used to remove the provider at a later time.
    pub fn add_provider(
        &mut self,
        provider: Box<dyn ResourceManagerProvider>,
        order: i32,
        identifier: &str,
    ) {
        let io_thread_id = ThreadId::IO;
        if currently_on(io_thread_id) == 0 {
            let mut task = AddProvider::new(
                self.weak_self.clone(),
                Arc::new(Mutex::new(Some(provider))),
                order,
                identifier.to_string(),
            );
            post_task(io_thread_id, Some(&mut task));
            return;
        }

        let provider_entry = ProviderEntry {
            provider: Some(provider),
            order,
            identifier: identifier.to_string(),
            ..Default::default()
        };

        if self.providers.is_empty() {
            self.providers.push_back(provider_entry);
            return;
        }

        // Insert before the first entry with a higher `order` value.
        let index = self.providers.partition_point(|entry| entry.order < order);
        self.providers.insert(index, provider_entry);
    }

    /// Remove all providers with the specified `identifier` value. If any removed providers have
    /// pending requests the [RequestManagerProvider::on_request_cancel] method will be called. The
    /// removed providers may be deleted immediately or at a later time.
    pub fn remove_providers(&mut self, identifier: &str) {
        let io_thread_id = ThreadId::IO;
        if currently_on(io_thread_id) == 0 {
            let mut task = RemoveProviders::new(self.weak_self.clone(), identifier.to_string());
            post_task(io_thread_id, Some(&mut task));
            return;
        }

        if self.providers.is_empty() {
            return;
        }

        let mut index = 0;
        while index < self.providers.len() {
            index = if self.providers[index].identifier == identifier {
                self.delete_provider(index, false)
            } else {
                index + 1
            };
        }
    }

    /// Remove all providers.  If any removed providers have pending requests the [RequestManagerProvider::on_request_cancel]
    /// method will be called. The removed providers may be deleted immediately or at a later time.
    pub fn remove_all_providers(&mut self) {
        let io_thread_id = ThreadId::IO;
        if currently_on(io_thread_id) == 0 {
            let mut task = RemoveAllProviders::new(self.weak_self.clone());
            post_task(io_thread_id, Some(&mut task));
            return;
        }

        if self.providers.is_empty() {
            return;
        }

        let mut index = 0;
        while index < self.providers.len() {
            index = self.delete_provider(index, true);
        }
    }

    /// Set the url filter. If not set the default no-op filter will be used. Changes to this value
    /// will not affect currently pending requests.
    pub fn set_url_filter(&mut self, filter: Option<UrlFilter>) {
        let io_thread_id = ThreadId::IO;
        if currently_on(io_thread_id) == 0 {
            let mut task =
                SetUrlFilter::new(self.weak_self.clone(), Arc::new(Mutex::new(Some(filter))));
            post_task(io_thread_id, Some(&mut task));
            return;
        }

        self.url_filter = Arc::new(filter.unwrap_or(Box::new(get_filtered_url)));
    }

    /// Set the mime type resolver. If not set the default resolver will be used. Changes to this
    /// value will not affect currently pending requests.
    pub fn set_mime_type_resolver(&mut self, resolver: Option<MimeTypeResolver>) {
        let io_thread_id = ThreadId::IO;
        if currently_on(io_thread_id) == 0 {
            let mut task = SetMimeTypeResolver::new(
                self.weak_self.clone(),
                Arc::new(Mutex::new(Some(resolver))),
            );
            post_task(io_thread_id, Some(&mut task));
            return;
        }

        self.mime_type_resolver = Arc::new(resolver.unwrap_or(Box::new(get_mime_type)));
    }

    /// Called from [RequestHandler::on_before_resource_load] on the browser process IO thread.
    pub fn on_before_resource_load(
        &mut self,
        browser: Browser,
        frame: Frame,
        request: Request,
        callback: Callback,
    ) -> ReturnValue {
        debug_assert_ne!(
            currently_on(ThreadId::IO),
            0,
            "on_before_resource_load must be called on the IO thread"
        );

        let first_entry = self.get_next_valid_provider(0);
        debug_assert!(first_entry <= self.providers.len());
        if first_entry == self.providers.len() {
            // No providers so continue the request immediately.
            return ReturnValue::CONTINUE;
        }

        let url = CefString::from(&request.url()).to_string();
        let url = (*self.url_filter)(&url);
        let url = get_url_without_query_or_fragment(&url).to_string();
        let state = RequestState {
            manager: self.weak_self.clone(),
            callback: Some(callback),
            current_entry: first_entry,
            current_request: 0,
            params: Arc::new(RequestParams {
                url,
                browser,
                frame,
                request,
                url_filter: self.url_filter.clone(),
                mime_type_resolver: self.mime_type_resolver.clone(),
            }),
        };

        ReturnValue::from(if self.send_request(state) {
            // If the request is potentially handled we need to continue asynchronously.
            sys::cef_return_value_t::RV_CONTINUE_ASYNC
        } else {
            sys::cef_return_value_t::RV_CONTINUE
        })
    }

    /// Called from [RequestHandler::resource_handler] on the browser process IO thread.
    pub fn resource_handler(
        &mut self,
        _browser: Browser,
        _frame: Frame,
        request: Request,
    ) -> Option<ResourceHandler> {
        debug_assert_ne!(
            currently_on(ThreadId::IO),
            0,
            "resource_handler must be called on the IO thread"
        );

        self.pending_handlers.remove(&request.identifier())
    }

    /// Send the request to providers in order until one potentially handles it or we run out of
    /// providers. Returns true if the request is potentially handled.
    fn send_request(&mut self, state: RequestState) -> bool {
        let mut potentially_handled = false;

        let current_entry = state.current_entry;
        let params = state.params.clone();
        let mut state = Some(state);

        while state.is_some() {
            debug_assert!(
                current_entry < self.providers.len(),
                "Should not be on the last provider entry."
            );
            let request = ResourceManagerRequest::new(state.take(), params.clone());
            let Ok(mut request) = request.lock() else {
                break;
            };

            // Give the provider an opportunity to handle the request.
            state = request.send_request();

            let Some(mut next_state) = state.take() else {
                potentially_handled = true;
                break;
            };

            // The provider will not handle the request. Move to the next provider if any.
            if self.increment_provider(&mut next_state) {
                state = Some(next_state);
            } else {
                self.stop_request(next_state);
            }
        }

        potentially_handled
    }

    fn continue_request(&mut self, mut state: RequestState, handler: Option<ResourceHandler>) {
        debug_assert_ne!(
            currently_on(ThreadId::IO),
            0,
            "continue_request must be called on the IO thread"
        );

        if let Some(handler) = handler {
            // The request has been handled. Associate the request ID with the handler.
            self.pending_handlers
                .insert(state.params.request.identifier(), handler);
            self.stop_request(state);
        } else {
            // Move to the next provider if any.
            if self.increment_provider(&mut state) {
                self.send_request(state);
            } else {
                self.stop_request(state);
            }
        }
    }

    fn stop_request(&mut self, mut state: RequestState) {
        debug_assert_ne!(
            currently_on(ThreadId::IO),
            0,
            "stop_request must be called on the IO thread"
        );

        self.detach_request_from_provider(&mut state);
    }

    /// Move state to the next provider if any and return true if there are more providers.
    fn increment_provider(&mut self, state: &mut RequestState) -> bool {
        let next_entry = self.get_next_valid_provider(state.current_entry);
        self.detach_request_from_provider(state);
        if next_entry < self.providers.len() {
            state.current_entry = next_entry;
            true
        } else {
            false
        }
    }

    /// The new provider, if any, should be determined before calling this method.
    fn detach_request_from_provider(&mut self, state: &mut RequestState) {
        // Remove the association from the current provider entry.
        let Some(current_entry) = self.providers.get_mut(state.current_entry) else {
            return;
        };
        current_entry.pending_requests.remove(state.current_request);
        if current_entry.deletion_pending && current_entry.pending_requests.is_empty() {
            // Delete the current provider entry now.
            self.providers.remove(state.current_entry);
        }
        // Set to the end for error checking purposes.
        state.current_entry = self.providers.len();
    }

    /// Move to the next provider that is not pending deletion.
    fn get_next_valid_provider(&self, current_provider: usize) -> usize {
        self.providers
            .iter()
            .enumerate()
            .skip(current_provider)
            .find(|&(_, entry)| !entry.deletion_pending)
            .map(|(index, _)| index)
            .unwrap_or(self.providers.len())
    }

    fn delete_provider(&mut self, deleted_provider: usize, stop: bool) -> usize {
        debug_assert_ne!(
            currently_on(ThreadId::IO),
            0,
            "delete_provider must be called on the IO thread"
        );
        let Some(current_entry) = self.providers.get_mut(deleted_provider) else {
            return self.providers.len();
        };
        if current_entry.deletion_pending {
            return deleted_provider;
        }
        if !current_entry.pending_requests.is_empty() {
            // Don't delete the provider entry until all pending requests have cleared.
            current_entry.deletion_pending = true;

            // Continue pending requests immediately.
            for request in current_entry.pending_requests.iter() {
                let Ok(mut request_lock) = request.lock() else {
                    continue;
                };
                if request_lock.state.is_some() {
                    if stop {
                        request_lock.stop_request();
                    } else {
                        request_lock.continue_request(None);
                    }
                    if let Some(provider) = &current_entry.provider {
                        provider.on_request_canceled(request.clone());
                    }
                }
            }

            deleted_provider + 1
        } else {
            // Delete the provider entry now.
            self.providers.remove(deleted_provider);
            deleted_provider
        }
    }
}

/// Returns `url` without the query or fragment components, if any.
pub fn get_url_without_query_or_fragment(url: &str) -> &str {
    url.split(['?', '#']).next().unwrap_or(url)
}

/// Determine the mime type based on the `url` file extension.
pub fn get_mime_type(url: &str) -> String {
    let url = get_url_without_query_or_fragment(url);
    url.rsplit('.')
        .next()
        .map(|extension| {
            let extension = CefString::from(extension);
            CefString::from(&crate::get_mime_type(Some(&extension))).to_string()
        })
        .unwrap_or_else(|| String::from("text/html"))
}

/// Default no-op filter.
pub fn get_filtered_url(url: &str) -> String {
    url.to_string()
}

/// Normalize the URL path by adding a trailing slash if it's missing.
pub fn normalize_url_path(url_path: &str) -> String {
    format!("{}/", url_path.trim_end_matches('/'))
}

pub fn get_file_relative_path(
    mut base_path: PathBuf,
    url_path: &str,
    url: &str,
) -> Option<PathBuf> {
    let segments = url.strip_prefix(url_path)?.split('/');
    for segment in segments {
        base_path.push(segment)
    }
    Some(base_path)
}
