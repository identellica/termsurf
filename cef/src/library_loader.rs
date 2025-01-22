use crate::{load_library, unload_library};

pub struct LibraryLoader {
    path: std::path::PathBuf,
}

impl LibraryLoader {
    const FRAMEWORK_PATH: &str =
        "Chromium Embedded Framework.framework/Chromium Embedded Framework";

    pub fn new(path: &std::path::Path, helper: bool) -> Self {
        let resolver = if helper {
            "../../../.."
        } else {
            "../../Frameworks"
        };
        let path = path
            .join(resolver)
            .join(Self::FRAMEWORK_PATH)
            .canonicalize()
            .unwrap();

        Self { path }
    }

    // See [cef_load_library] for more documentation.
    pub fn load(&self) -> bool {
        Self::load_library(&self.path)
    }

    fn load_library(name: &std::path::Path) -> bool {
        use std::os::unix::ffi::OsStrExt;
        let Ok(name) = std::ffi::CString::new(name.as_os_str().as_bytes()) else {
            return false;
        };
        unsafe { load_library(Some(&*name.as_ptr().cast())) == 1 }
    }
}

impl Drop for LibraryLoader {
    fn drop(&mut self) {
        if unload_library() != 1 {
            eprintln!("cannot unload framework {}", self.path.display());
        }
    }
}
