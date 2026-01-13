#![cfg_attr(
    all(not(debug_assertions), not(feature = "sandbox"), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod shared;

#[cfg(target_os = "macos")]
mod mac;

#[cfg(not(all(feature = "sandbox", target_os = "windows")))]
fn main() -> Result<(), &'static str> {
    let _library = shared::load_cef();

    let args = cef::args::Args::new();
    let Some(cmd_line) = args.as_cmd_line() else {
        return Err("Failed to parse command line arguments");
    };

    shared::run_main(args.as_main_args(), &cmd_line, std::ptr::null_mut());
    Ok(())
}

#[cfg(all(feature = "sandbox", target_os = "windows"))]
fn main() -> Result<(), &'static str> {
    Err("Running in sandbox mode on Windows requires bootstrap.exe or bootstrapc.exe.")
}
