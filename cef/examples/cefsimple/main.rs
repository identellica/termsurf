use cef::{args::Args, rc::*, *};

struct DemoApp(*mut RcImpl<cef_sys::_cef_app_t, Self>);

impl DemoApp {
    fn new() -> App {
        App::new(Self(std::ptr::null_mut()))
    }
}

impl WrapApp for DemoApp {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_sys::_cef_app_t, Self>) {
        self.0 = object;
    }
}

impl Clone for DemoApp {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }

        Self(self.0)
    }
}

impl Rc for DemoApp {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

struct DemoBrowserProcessHandler(*mut RcImpl<cef_sys::cef_browser_process_handler_t, Self>);
impl DemoBrowserProcessHandler {
    fn new() -> Self {
        Self(std::ptr::null_mut())
    }
}
impl Rc for DemoBrowserProcessHandler {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}
impl WrapBrowserProcessHandler for DemoBrowserProcessHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_sys::_cef_browser_process_handler_t, Self>) {
        self.0 = object;
    }
}
impl Clone for DemoBrowserProcessHandler {
    fn clone(&self) -> Self {
        unsafe {
            let rc = &mut *self.0;
            rc.interface.add_ref();
            Self(self.0)
        }
    }
}

impl ImplBrowserProcessHandler for DemoBrowserProcessHandler {
    fn get_raw(&self) -> *mut cef_sys::_cef_browser_process_handler_t {
        self.0.cast()
    }

    // The real lifespan of cef starts from `on_context_initialized`, so all the cef objects should be manipulated after that.
    fn on_context_initialized(&self) {
        println!("cef context intiialized");
        let mut client = DemoClient::new();
        let url = CefString::from(&CefStringUtf8::from("https://www.google.com"));

        browser_host_create_browser_sync(
            Some(&Default::default()),
            Some(&mut client),
            Some(&url),
            Some(&Default::default()),
            Option::<&mut DictionaryValue>::None,
            Option::<&mut RequestContext>::None,
        )
        .expect("Failed to create browser view");
    }
}

impl ImplApp for DemoApp {
    fn get_raw(&self) -> *mut cef_sys::_cef_app_t {
        self.0 as *mut cef_sys::_cef_app_t
    }

    fn get_browser_process_handler(&self) -> Option<BrowserProcessHandler> {
        BrowserProcessHandler::new(DemoBrowserProcessHandler::new()).into()
    }
}

struct DemoClient(*mut RcImpl<cef_sys::_cef_client_t, Self>);

impl DemoClient {
    fn new() -> Client {
        Client::new(Self(std::ptr::null_mut()))
    }
}

impl WrapClient for DemoClient {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_sys::_cef_client_t, Self>) {
        self.0 = object;
    }
}

impl Clone for DemoClient {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }

        Self(self.0)
    }
}

impl Rc for DemoClient {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplClient for DemoClient {
    fn get_raw(&self) -> *mut cef_sys::_cef_client_t {
        self.0 as *mut cef_sys::_cef_client_t
    }
}

struct DemoWindowDelegate {
    base: *mut RcImpl<cef_sys::_cef_window_delegate_t, Self>,
    browser_view: BrowserView,
}

impl DemoWindowDelegate {
    fn new(browser_view: BrowserView) -> WindowDelegate {
        WindowDelegate::new(Self {
            base: std::ptr::null_mut(),
            browser_view,
        })
    }
}

impl WrapWindowDelegate for DemoWindowDelegate {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_sys::_cef_window_delegate_t, Self>) {
        self.base = object;
    }
}

impl Clone for DemoWindowDelegate {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.base;
            rc_impl.interface.add_ref();
        }

        Self {
            base: self.base,
            browser_view: self.browser_view.clone(),
        }
    }
}

impl Rc for DemoWindowDelegate {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.base;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplViewDelegate for DemoWindowDelegate {
    fn on_child_view_changed(
        &self,
        _view: Option<&mut impl ImplView>,
        _added: ::std::os::raw::c_int,
        _child: Option<&mut impl ImplView>,
    ) {
        // view.as_panel().map(|x| x.as_window().map(|w| w.close()));
    }

    fn get_raw(&self) -> *mut cef_sys::_cef_view_delegate_t {
        self.base as *mut cef_sys::_cef_view_delegate_t
    }
}

impl ImplPanelDelegate for DemoWindowDelegate {}

impl ImplWindowDelegate for DemoWindowDelegate {
    fn on_window_created(&self, window: Option<&mut impl ImplWindow>) {
        if let Some(window) = window {
            let mut view = self.browser_view.clone();
            window.add_child_view(Some(&mut view));
            window.show();
        }
    }

    fn can_close(&self, _window: Option<&mut impl ImplWindow>) -> ::std::os::raw::c_int {
        1
    }

    fn on_window_destroyed(&self, _window: Option<&mut impl ImplWindow>) {
        quit_message_loop();
    }
}

// FIXME: Rewrite this demo based on cef/tests/cefsimple
fn main() {
    #[cfg(target_os = "macos")]
    let loader = library_loader::LibraryLoader::new(&std::env::current_exe().unwrap(), false);
    #[cfg(target_os = "macos")]
    assert!(loader.load());

    let args = Args::new(std::env::args());

    let cmd = command_line_create().unwrap();
    #[cfg(not(target_os = "windows"))]
    cmd.init_from_argv(args.as_main_args().argc, args.as_main_args().argv.cast());

    /* cmd must be init'ed from string on windows
    #[cfg(target_os = "windows")]
    cmd.init_from_string();
    */

    let is_browser_process = cmd.has_switch(Some(&"type".into())) != 1;

    let mut app = DemoApp::new();

    let ret = execute_process(
        Some(args.as_main_args()),
        Some(&mut app),
        std::ptr::null_mut(),
    );

    if is_browser_process {
        println!("launch browser process");
        assert!(ret == -1, "cannot execute browser process");
    } else {
        let process_type = cmd
            .get_switch_value(Some(&"type".into()))
            .as_ref()
            .map(CefStringUtf8::from)
            .unwrap();
        println!("launch process {process_type}");
        assert!(ret >= 0, "cannot execute non-browser process");
        // non-browser process does not initialize cef
        return;
    }
    let mut settings = Settings::default();
    settings.no_sandbox = true as _;
    assert_eq!(
        initialize(
            Some(args.as_main_args()),
            Some(&settings),
            Some(&mut app),
            std::ptr::null_mut()
        ),
        1
    );

    run_message_loop();
    shutdown();
}
