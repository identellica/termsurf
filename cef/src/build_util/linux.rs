use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0:?}")]
    Io(#[from] io::Error),
    #[error("Metadata error: {0:?}")]
    Metadata(#[from] super::metadata::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// See https://bitbucket.org/chromiumembedded/cef/wiki/GeneralUsage.md#markdown-header-linux
pub fn bundle(app_path: &Path, target_path: &Path, executable_name: &str) -> Result<PathBuf> {
    let cef_path = cef_dll_sys::get_cef_dir().unwrap();
    copy_directory(&cef_path, &app_path)?;

    const LOCALES_DIR: &str = "locales";
    copy_directory(&cef_path.join(LOCALES_DIR), &app_path.join(LOCALES_DIR))?;

    copy_app(app_path, target_path, executable_name)
}

/// Similar to [`bundle`], but this will invoke `cargo build` to build the executable target.
pub fn build_bundle(app_path: &Path, executable_name: &str, release: bool) -> Result<PathBuf> {
    let cargo_metadata = super::metadata::get_cargo_metadata()?;
    let target_path =
        cargo_metadata
            .target_directory()
            .join(if release { "release" } else { "debug" });

    cargo_build(executable_name, release)?;

    bundle(app_path, &target_path, executable_name)
}

fn copy_app(app_path: &Path, target_path: &Path, executable_name: &str) -> Result<PathBuf> {
    let executable_path = app_path.join(executable_name);
    let target_executable = target_path.join(executable_name);
    fs::copy(&target_executable, &executable_path)?;
    Ok(executable_path)
}

fn copy_directory(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let dst_path = dst.join(entry.file_name());
        if entry.file_type()?.is_file() {
            fs::copy(entry.path(), &dst_path)?;
        }
    }
    Ok(())
}

fn cargo_build(name: &str, release: bool) -> Result<()> {
    println!("Building {name}...");

    let mut args = vec!["build"];
    if release {
        args.push("--release");
    }
    #[cfg(feature = "linux-x11")]
    args.extend(["-F", "linux-x11"]);
    args.extend(["--bin", name]);

    let status = Command::new(super::cargo_path()).args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::from(io::ErrorKind::Interrupted).into())
    }
}
