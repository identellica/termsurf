use crate::browser::util_win::*;
use cef::{
    wrapper::{byte_read_handler::*, resource_manager::*, stream_resource_handler::*},
    *,
};
use std::{
    mem,
    sync::{Arc, Mutex, OnceLock},
};
use windows_sys::Win32::System::LibraryLoader::{
    FindResourceW, LoadResource, LockResource, SizeofResource,
};

pub type GetResourceId = Box<dyn Send + Sync + Fn(&str) -> u16>;

static INSTANCE: OnceLock<Arc<Mutex<Option<GetResourceId>>>> = OnceLock::new();

pub fn get_fn_get_resource_id() -> Arc<Mutex<Option<GetResourceId>>> {
    INSTANCE.get_or_init(|| Arc::new(Mutex::new(None))).clone()
}

pub fn set_fn_get_resource_id(mut get_resource_id: Option<GetResourceId>) -> Option<GetResourceId> {
    let instance = get_fn_get_resource_id();
    let Ok(mut instance) = instance.lock() else {
        return get_resource_id;
    };
    mem::swap(&mut *instance, &mut get_resource_id);
    get_resource_id
}

pub fn load_binary_resource(resource_name: &str) -> Option<Vec<u8>> {
    let get_resource_id = get_fn_get_resource_id();
    let get_resource_id = get_resource_id.lock().ok()?;
    let get_resource_id = get_resource_id.as_ref()?;

    let resource_id = (*get_resource_id)(resource_name);
    let instance = get_code_module_handle();

    unsafe {
        // Defined in https://github.com/chromiumembedded/cef/blob/master/tests/cefclient/browser/resource.h
        const RT_BINARY: u16 = 256;

        let resource = FindResourceW(
            instance,
            resource_id as usize as *const _,
            RT_BINARY as usize as *const _,
        );
        if resource.is_null() {
            return None;
        }

        let data = LoadResource(instance, resource);
        if data.is_null() {
            return None;
        }

        let size = SizeofResource(instance, resource);
        if size == 0 {
            return None;
        }

        let ptr = LockResource(data);
        if ptr.is_null() {
            return None;
        }

        Some(std::slice::from_raw_parts(ptr as *const u8, size as usize).to_vec())
    }
}

pub fn get_binary_resource_reader(resource_name: &str) -> Option<StreamReader> {
    let data = load_binary_resource(resource_name)?;
    let stream = ByteStream::new(data);
    let mut handler = ByteReadHandler::new(Arc::new(Mutex::new(stream)));
    stream_reader_create_for_handler(Some(&mut handler))
}

struct BinaryResourceProvider {
    url_path: String,
    resource_path_prefix: String,
}

impl BinaryResourceProvider {
    fn new(url_path: &str, resource_path_prefix: &str) -> Self {
        Self {
            url_path: normalize_url_path(url_path),
            resource_path_prefix: normalize_url_path(resource_path_prefix),
        }
    }
}

impl ResourceManagerProvider for BinaryResourceProvider {
    fn on_request(&self, request: Arc<Mutex<ResourceManagerRequest>>) -> bool {
        assert_ne!(
            currently_on(ThreadId::IO),
            0,
            "on_request must be called on the IO thread"
        );

        let Ok(mut request) = request.lock() else {
            return false;
        };
        let url = request.url();
        let Some(relative_path) = url.strip_prefix(self.url_path.as_str()) else {
            // Not handled by this provider.
            return false;
        };

        let mime_type = request.mime_type_resolver()(url);
        let relative_path = format!("{}/{relative_path}", self.resource_path_prefix);
        let handler = get_binary_resource_reader(&relative_path)
            .map(|stream| StreamResourceHandler::new_with_stream(mime_type, stream));

        request.continue_request(handler);
        true
    }
}

pub fn create_binary_resource_provider(
    url_path: &str,
    resource_path_prefix: &str,
) -> Box<dyn ResourceManagerProvider> {
    Box::new(BinaryResourceProvider::new(url_path, resource_path_prefix))
}
