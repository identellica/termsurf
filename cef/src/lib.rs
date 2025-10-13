#![doc = include_str!("../README.md")]

pub mod args;
pub mod rc;
pub mod string;

#[cfg(target_os = "macos")]
pub mod application_mac;

#[cfg(target_os = "macos")]
pub mod library_loader;

#[cfg(target_os = "macos")]
pub mod sandbox;

#[cfg(feature = "accelerated_osr")]
pub mod osr_texture_import;

#[rustfmt::skip]
mod bindings;
pub use bindings::*;

pub use cef_dll_sys as sys;

#[cfg(all(
    not(any(target_os = "macos", target_os = "windows", target_os = "linux")),
    feature = "accelerated_osr"
))]
compile_error!("accelerated_osr not supported on this platform");
