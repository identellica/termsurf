//! # cef-rs
//!
//! Use the [Chromium Embedded Framework](https://github.com/chromiumembedded/cef) in Rust.

pub mod args;
pub mod rc;
pub mod sandbox_info;
pub mod string;

#[cfg(target_os = "macos")]
pub mod library_loader;

#[rustfmt::skip]
mod bindings;
pub use bindings::*;

pub use cef_dll_sys as sys;
