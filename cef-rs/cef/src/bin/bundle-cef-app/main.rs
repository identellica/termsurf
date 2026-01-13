#[cfg(not(feature = "build-util"))]
fn main() {
    let command = std::env::current_exe().expect("Failed to get current executable path");
    eprintln!("Disabled: {command:?} was compiled without the build-util feature.");
}

#[cfg(all(feature = "build-util", target_os = "macos"))]
mod mac;

#[cfg(all(feature = "build-util", target_os = "macos"))]
fn main() -> anyhow::Result<()> {
    mac::main()
}

#[cfg(all(feature = "build-util", target_os = "linux"))]
mod linux;

#[cfg(all(feature = "build-util", target_os = "linux"))]
fn main() -> anyhow::Result<()> {
    linux::main()
}

#[cfg(all(feature = "build-util", target_os = "windows"))]
mod win;

#[cfg(all(feature = "build-util", target_os = "windows"))]
fn main() -> anyhow::Result<()> {
    win::main()
}
