#![doc = include_str!("../README.md")]

pub mod args;
pub mod rc;
pub mod string;

#[cfg(target_os = "macos")]
pub mod library_loader;

#[cfg(target_os = "macos")]
pub mod sandbox;

#[rustfmt::skip]
mod bindings;
pub use bindings::*;

pub use cef_dll_sys as sys;
