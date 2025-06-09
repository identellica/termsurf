#[cfg(not(target_os = "windows"))]
use std::ffi::{c_char, CString};

use crate::*;

#[derive(Clone, Default)]
pub struct Args {
    #[cfg(not(target_os = "windows"))]
    _source: Vec<CString>,
    #[cfg(not(target_os = "windows"))]
    _argv: Vec<*const c_char>,
    main_args: MainArgs,
    cmd_line: Option<CommandLine>,
}

impl Args {
    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        use std::ffi::CStr;

        let main_args = MainArgs {
            instance: cef_dll_sys::HINSTANCE(
                unsafe {
                    windows_sys::Win32::System::LibraryLoader::GetModuleHandleW(std::ptr::null())
                }
                .cast(),
            ),
        };
        let cmd_line = command_line_create().and_then(|cmd_line| {
            unsafe {
                CStr::from_ptr(windows_sys::Win32::System::Environment::GetCommandLineA().cast())
            }
            .to_str()
            .ok()
            .map(|args| {
                cmd_line.init_from_string(Some(&CefString::from(args)));
                cmd_line
            })
        });

        Self {
            main_args,
            cmd_line,
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn new() -> Self {
        let args = std::env::args();
        let _source = args
            .into_iter()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<CString>>();
        let _argv = _source
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();
        let main_args = MainArgs {
            argc: _argv.len() as i32,
            argv: _argv.as_ptr() as *mut *mut _,
        };
        let cmd_line = command_line_create().inspect(|cmd_line| {
            if !_argv.is_empty() {
                cmd_line.init_from_argv(_argv.len() as i32, _argv.as_ptr());
            }
        });

        Self {
            _source,
            _argv,
            main_args,
            cmd_line,
        }
    }

    pub fn as_main_args(&self) -> &MainArgs {
        &self.main_args
    }

    pub fn as_cmd_line(&self) -> Option<&CommandLine> {
        self.cmd_line.as_ref()
    }
}
