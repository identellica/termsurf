#![doc = include_str!("../../README.md")]

pub mod args;
pub mod rc;
pub mod string;

#[cfg(target_os = "macos")]
pub mod library_loader;

#[rustfmt::skip]
mod bindings;
pub use bindings::*;

pub use cef_sys as sys;
