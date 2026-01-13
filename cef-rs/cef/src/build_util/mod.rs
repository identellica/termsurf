use std::env;

pub mod metadata;

#[cfg(target_os = "macos")]
pub mod mac;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod win;

/// Prefer the path in the `CARGO` environment variable if specified, otherwise just execute
/// `cargo` from wherever it is found in the `PATH`.
fn cargo_path() -> String {
    env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}
