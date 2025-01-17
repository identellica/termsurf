use crate::dirs;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const TARGETS: &[&str] = &[
    // macos
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    // windows
    "aarch64-pc-windows-msvc",
    "x86_64-pc-windows-msvc",
    "i686-pc-windows-msvc",
    // linux
    "x86_64-unknown-linux-gnu",
    "i686-unknown-linux-gnu",
    "arm-unknown-linux-gnueabi",
    "aarch64-unknown-linux-gnu",
];

pub fn download(target: &str) -> PathBuf {
    assert!(TARGETS.contains(&target), "unsupported target {target}");
    let (os, arch) = target_to_os_arch(target);
    let cef_path = dirs::get_cef_root(os, arch);

    let archive = download_cef::download_target_archive(
        target,
        env!("CARGO_PKG_VERSION"),
        dirs::get_out_dir(),
        true,
    )
    .expect("download failed");
    let archive_dir =
        download_cef::extract_target_archive(target, &archive, dirs::get_out_dir(), true)
            .expect("extraction failed");

    build_cef_dll_wrapper(&cef_path, &archive_dir, os);

    // rename cef_sandbox.a to libcef_sandbox.a, since we cannot link the library without lib
    // prefix, see https://www.reddit.com/r/rust/comments/dzj650/linking_against_lib_which_file_name_doesnt_start
    if os == "macos" {
        fs::rename(
            cef_path.join("cef_sandbox.a"),
            cef_path.join("libcef_sandbox.a"),
        )
        .expect("failed to rename cef_sandbox.a");
    }

    archive_dir
}

pub fn sys_bindgen(target: &str) -> crate::Result<()> {
    assert!(TARGETS.contains(&target), "unsupported target {target}");
    let (os, arch) = target_to_os_arch(target);
    let cef_path = dirs::get_cef_root(os, arch);
    bindgen(target, &cef_path)
}

pub fn get_target_bindings(target: &str) -> String {
    assert!(TARGETS.contains(&target), "unsupported target {target}");
    format!("{}.rs", target.replace('-', "_"))
}

fn bindgen(target: &str, cef_path: &Path) -> crate::Result<()> {
    let mut sys_bindings = dirs::get_sys_dir()?;
    let mut wrapper = sys_bindings.clone();
    sys_bindings.push("src");
    sys_bindings.push("bindings");
    sys_bindings.push(format!("{}.rs", target.replace('-', "_")));
    wrapper.push("wrapper.h");

    let mut bindings = bindgen::Builder::default()
        .header(wrapper.display().to_string())
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .allowlist_type("cef_.*")
        .allowlist_function("cef_.*")
        .bitfield_enum(".*_mask_t")
        .clang_args([
            format!("-I{}", cef_path.display()),
            format!("--target={target}"),
        ]);

    if target.contains("windows") {
        bindings = bindings.new_type_alias("HINSTANCE").new_type_alias("HWND");
    } else if target.contains("apple") {
        let sdk_path = Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-path"])
            .output()
            .unwrap()
            .stdout;

        bindings = bindings.clang_arg(format!(
            "--sysroot={}",
            String::from_utf8_lossy(&sdk_path).trim()
        ));
    }

    let bindings = bindings.generate()?;

    bindings.write_to_file(&sys_bindings)?;
    Ok(())
}

fn build_cef_dll_wrapper(cef_path: &Path, archive_dir: &Path, os: &str) {
    if os != "macos" {
        return;
    }

    let lib_name = format!(
        "libcef_dll_wrapper.{}",
        if os == "windows" { "lib" } else { "a" }
    );
    if cef_path.join(&lib_name).exists() {
        println!("cef: {lib_name} already exists, skip building");
        return;
    }

    let build_dir = archive_dir.join("build");
    fs::create_dir_all(&build_dir).unwrap();

    Command::new("cmake")
        .current_dir(&build_dir)
        .args([
            "-G",
            "Ninja",
            "-DCMAKE_OBJECT_PATH_MAX=500",
            "-DCMAKE_BUILD_TYPE=RelWithDebInfo",
            "..",
        ])
        .output()
        .unwrap();

    Command::new("ninja")
        .current_dir(&build_dir)
        .arg("libcef_dll_wrapper")
        .output()
        .unwrap();

    fs::copy(
        build_dir.join("libcef_dll_wrapper").join(&lib_name),
        cef_path.join(&lib_name),
    )
    .unwrap();
}

fn target_to_os_arch(target: &str) -> (&str, &str) {
    match target {
        "aarch64-apple-darwin" => ("macos", "aarch64"),
        "x86_64-apple-darwin" => ("macos", "x86_64"),
        "i686-pc-windows-msvc" => ("windows", "x86"),
        "x86_64-pc-windows-msvc" => ("windows", "x86_64"),
        "aarch64-pc-windows-msvc" => ("windows", "aarch64"),
        "x86_64-unknown-linux-gnu" => ("linux", "x86_64"),
        "i686-unknown-linux-gnu" => ("linux", "x86"),
        "arm-unknown-linux-gnueabi" => ("linux", "arm"),
        "aarch64-unknown-linux-gnu" => ("linux", "aarch64"),
        v => panic!("unsupported {v:?}"),
    }
}
