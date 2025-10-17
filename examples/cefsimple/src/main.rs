use cef::{args::Args, rc::*, *};
use std::sync::{Arc, Mutex};

wrap_app! {
    struct DemoApp {
        window: Arc<Mutex<Option<Window>>>,
    }

    impl App {
        fn browser_process_handler(&self) -> Option<BrowserProcessHandler> {
            Some(DemoBrowserProcessHandler::new(
                self.window.clone(),
            ))
        }
    }
}

wrap_browser_process_handler! {
    struct DemoBrowserProcessHandler {
        window: Arc<Mutex<Option<Window>>>,
    }

    impl BrowserProcessHandler {
        // The real lifespan of cef starts from `on_context_initialized`, so all the cef objects should be manipulated after that.
        fn on_context_initialized(&self) {
            println!("cef context intiialized");
            let mut client = DemoClient::new();
            let url = CefString::from("https://www.google.com");

            let browser_view = browser_view_create(
                Some(&mut client),
                Some(&url),
                Some(&Default::default()),
                Option::<&mut DictionaryValue>::None,
                Option::<&mut RequestContext>::None,
                Option::<&mut BrowserViewDelegate>::None,
            )
            .expect("Failed to create browser view");

            let mut delegate = DemoWindowDelegate::new(browser_view);
            if let Ok(mut window) = self.window.lock() {
                *window = Some(
                    window_create_top_level(Some(&mut delegate)).expect("Failed to create window"),
                );
            }
        }
    }
}

wrap_client! {
    struct DemoClient;
    impl Client {}
}

wrap_window_delegate! {
    struct DemoWindowDelegate {
        browser_view: BrowserView,
    }

    impl ViewDelegate {
        fn on_child_view_changed(
            &self,
            _view: Option<&mut View>,
            _added: ::std::os::raw::c_int,
            _child: Option<&mut View>,
        ) {
            // view.as_panel().map(|x| x.as_window().map(|w| w.close()));
        }
    }

    impl PanelDelegate {}

    impl WindowDelegate {
        fn on_window_created(&self, window: Option<&mut Window>) {
            if let Some(window) = window {
                let view = self.browser_view.clone();
                window.add_child_view(Some(&mut (&view).into()));
                window.show();
            }
        }

        fn on_window_destroyed(&self, _window: Option<&mut Window>) {
            quit_message_loop();
        }

        fn with_standard_window_buttons(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
            1
        }

        fn can_resize(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
            1
        }

        fn can_maximize(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
            1
        }

        fn can_minimize(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
            1
        }

        fn can_close(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
            1
        }
    }
}

// FIXME: Rewrite this demo based on cef/tests/cefsimple
fn main() {
    #[cfg(target_os = "macos")]
    let _loader = {
        let loader = library_loader::LibraryLoader::new(&std::env::current_exe().unwrap(), false);
        assert!(loader.load());
        loader
    };

    #[cfg(target_os = "macos")]
    cef::application_mac::SimpleApplication::init().unwrap();

    let _ = api_hash(sys::CEF_API_VERSION_LAST, 0);

    let args = Args::new();
    let cmd = args.as_cmd_line().unwrap();

    let switch = CefString::from("type");
    let is_browser_process = cmd.has_switch(Some(&switch)) != 1;

    let window = Arc::new(Mutex::new(None));
    let mut app = DemoApp::new(window.clone());

    let ret = execute_process(
        Some(args.as_main_args()),
        Some(&mut app),
        std::ptr::null_mut(),
    );

    if is_browser_process {
        println!("launch browser process");
        assert!(ret == -1, "cannot execute browser process");
    } else {
        let process_type = CefString::from(&cmd.switch_value(Some(&switch)));
        println!("launch process {process_type}");
        assert!(ret >= 0, "cannot execute non-browser process");
        // non-browser process does not initialize cef
        return;
    }
    let settings = Settings {
        no_sandbox: !cfg!(feature = "sandbox") as _,
        ..Default::default()
    };
    assert_eq!(
        initialize(
            Some(args.as_main_args()),
            Some(&settings),
            Some(&mut app),
            std::ptr::null_mut(),
        ),
        1
    );

    run_message_loop();

    let window = window.lock().expect("Failed to lock window");
    let window = window.as_ref().expect("Window is None");
    assert!(window.has_one_ref());

    shutdown();
}
