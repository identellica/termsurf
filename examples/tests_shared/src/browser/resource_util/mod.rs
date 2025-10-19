#[cfg(target_os = "windows")]
pub mod win;
#[cfg(target_os = "windows")]
pub use win::*;

#[cfg(not(target_os = "windows"))]
pub mod posix;
#[cfg(not(target_os = "windows"))]
pub use posix::*;
