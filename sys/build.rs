#[cfg(not(feature = "dox"))]
fn main() -> Result<(), String> {
    println!("cargo:rerun-if-change=build.rs");
    let path = std::env::var("FLATPAK")
        .map(|_| String::from("/usr/lib"))
        .or_else(|_| std::env::var("CEF_PATH"))
        .or_else(|_| {
            std::env::var("HOME").map(|mut val| {
                val.push_str("/.local/share/cef");
                val
            })
        })
        .map_err(|e| format!("Couldn't get the path of shared library: {e}"))?;

    let path = std::path::PathBuf::from(path).canonicalize().unwrap();
    let path = path.display();
    println!("cargo:rerun-if-change={path}");
    println!("cargo::rustc-link-search={path}");

    match std::env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("linux") => {
            println!("cargo::rustc-link-lib=dylib=cef");
        }
        Ok("windows") => {
            println!("cargo::rustc-link-lib=dylib=libcef");
        }
        Ok("macos") => {
            println!("cargo::rustc-link-lib=framework=AppKit");

            println!("cargo::rustc-link-lib=static=cef_dll_wrapper");
            println!("cargo::rustc-link-arg={path}/cef_sandbox.a");
        }
        os => unimplemented!("unknown target {}", os.unwrap_or("(unset)")),
    }

    Ok(())
}

#[cfg(feature = "dox")]
fn main() {}
