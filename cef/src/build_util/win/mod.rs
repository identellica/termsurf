use std::{
    fs,
    io::{self, Write},
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

const MANIFEST_CONTENT: &[u8] = include_bytes!("cef-app.exe.manifest");

fn copy_app(app_path: &Path, target_path: &Path, executable_name: &str) -> Result<PathBuf> {
    let mut manifest_file =
        fs::File::create(app_path.join(format!("{executable_name}.exe.manifest")))?;
    manifest_file.write_all(MANIFEST_CONTENT)?;

    #[cfg(feature = "sandbox")]
    {
        let dll_name = format!("{executable_name}.dll");
        let dll_path = app_path.join(&dll_name);
        let target_dll = target_path.join(&dll_name);
        fs::copy(&target_dll, &dll_path)?;

        let pdb_name = format!("{executable_name}.pdb");
        let pdb_path = app_path.join(&pdb_name);
        let target_pdb = target_path.join(&pdb_name);
        fs::copy(&target_pdb, &pdb_path)?;

        let executable_name = format!("{executable_name}.exe");
        let executable_path = app_path.join(&executable_name);
        let cef_path = cef_dll_sys::get_cef_dir().unwrap();
        let target_executable = cef_path.join("bootstrap.exe");
        fs::copy(&target_executable, &executable_path)?;

        Ok(executable_path)
    }

    #[cfg(not(feature = "sandbox"))]
    {
        let executable_name = format!("{executable_name}.exe");
        let executable_path = app_path.join(&executable_name);
        let target_executable = target_path.join(executable_name);
        fs::copy(&target_executable, &executable_path)?;

        Ok(executable_path)
    }
}

fn copy_directory(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let dst_path = dst.join(entry.file_name());
        if entry.file_type()?.is_file()
            && !entry
                .path()
                .extension()
                .map(|ext| ext == "exe")
                .unwrap_or_default()
        {
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
    #[cfg(feature = "sandbox")]
    args.extend_from_slice(&["-p", name, "--lib"]);
    #[cfg(not(feature = "sandbox"))]
    args.extend_from_slice(&["--no-default-features", "--bin", name]);

    let status = Command::new(super::cargo_path()).args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::from(io::ErrorKind::Interrupted).into())
    }
}
