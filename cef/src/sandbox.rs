use libloading::Library;
use std::{ffi::c_void, ptr::NonNull};

use crate::MainArgs;

pub struct Sandbox {
    lib: Library,
    context: Option<NonNull<c_void>>,
}

impl Sandbox {
    const LIBCEF_SANDBOX_PATH: &str =
        "../../../../Chromium Embedded Framework.framework/Libraries/libcef_sandbox.dylib";

    pub fn new() -> Self {
        unsafe {
            let lib = Library::new(
                std::env::current_exe()
                    .unwrap()
                    .join(Self::LIBCEF_SANDBOX_PATH)
                    .canonicalize()
                    .unwrap(),
            )
            .unwrap();
            Self { lib, context: None }
        }
    }

    pub fn initialize(&mut self, args: &MainArgs) {
        let inner = unsafe {
            self.lib
                .get::<extern "C" fn(
                    argc: std::os::raw::c_int,
                    argv: *mut *mut ::std::os::raw::c_char,
                ) -> *mut c_void>(b"cef_sandbox_initialize")
                .unwrap()(args.argc, args.argv)
        };
        self.context = NonNull::new(inner);
        assert!(self.context.is_some());
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        unsafe {
            if let Some(inner) = self.context {
                self.lib
                    .get::<extern "C" fn(context: *mut c_void)>(b"cef_sandbox_destroy")
                    .unwrap()(inner.as_ptr());
            }
        }
    }
}
