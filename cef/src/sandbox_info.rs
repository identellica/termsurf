use std::ptr;

pub struct SandboxInfo(*mut u8);

impl SandboxInfo {
    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        Self(crate::sandbox_info_create().cast())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn as_mut_ptr<T>(&self) -> *mut T {
        self.0.cast()
    }
}

impl Default for SandboxInfo {
    fn default() -> Self {
        Self(ptr::null_mut())
    }
}

#[cfg(target_os = "windows")]
impl Drop for SandboxInfo {
    fn drop(&mut self) {
        if !self.0.is_null() {
            crate::sandbox_info_destroy(self.0);
        }
    }
}
