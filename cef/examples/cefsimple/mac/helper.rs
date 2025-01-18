use cef::{rc::*, *};

struct DemoApp(*mut RcImpl<cef_dll_sys::_cef_app_t, Self>);

impl WrapApp for DemoApp {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_dll_sys::_cef_app_t, Self>) {
        self.0 = object;
    }
}

impl ImplApp for DemoApp {
    fn get_raw(&self) -> *mut cef_dll_sys::_cef_app_t {
        self.0 as *mut cef_dll_sys::_cef_app_t
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
    fn as_base(&self) -> &cef_dll_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

fn main() {
    let args = cef::args::Args::new(std::env::args());

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    let sandbox = sandbox_initialize(args.as_main_args().argc, args.as_main_args().argv);

    #[cfg(target_os = "macos")]
    let _loader = {
        let loader = library_loader::LibraryLoader::new(&std::env::current_exe().unwrap(), true);
        assert!(loader.load());
        loader
    };

    execute_process(
        Some(args.as_main_args()),
        None::<&mut DemoApp>,
        std::ptr::null_mut(),
    );

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    sandbox_destroy(sandbox.cast());
}
