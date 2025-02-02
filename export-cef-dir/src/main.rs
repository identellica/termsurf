use clap::Parser;
use download_cef::{CefIndex, OsAndArch};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

#[cfg(target_os = "windows")]
const DEFAULT_TARGET: &str = "x86_64-pc-windows-msvc";
#[cfg(target_os = "macos")]
const DEFAULT_TARGET: &str = "aarch64-apple-darwin";
#[cfg(target_os = "linux")]
const DEFAULT_TARGET: &str = "x86_64-unknown-linux-gnu";

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "false")]
    force: bool,
    #[arg(short, long, default_value = DEFAULT_TARGET)]
    target: String,
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let output = PathBuf::from(args.output);

    if output.exists() {
        if !args.force {
            return Err(anyhow::anyhow!(
                "target directory already exists: {}",
                output.display()
            ));
        }

        let parent = output.parent();
        let dir = output
            .file_name()
            .and_then(|dir| dir.to_str())
            .ok_or_else(|| anyhow::anyhow!("invalid target directory: {}", output.display()))?;
        let old_output = parent.map(|p| p.join(format!("old_{dir}")));
        let old_output = old_output.as_ref().unwrap_or(&output);
        fs::rename(&output, old_output)?;
        println!("Cleaning up: {}", old_output.display());
        fs::remove_dir_all(old_output)?
    }

    let target = args.target.as_str();
    let os_arch = OsAndArch::try_from(target)?;
    let out_dir = PathBuf::from(env!("OUT_DIR"));
    let cef_dir = os_arch.to_string();
    let cef_dir = out_dir.join(&cef_dir);

    if !fs::exists(&cef_dir)? {
        let cef_version = env!("CARGO_PKG_VERSION");
        let index = CefIndex::download()?;
        let platform = index.platform(target)?;
        let version = platform.version(cef_version)?;

        let archive = version.download_archive(&out_dir, true)?;
        let extracted_dir = download_cef::extract_target_archive(target, &archive, &out_dir, true)?;
        if extracted_dir != cef_dir {
            return Err(anyhow::anyhow!(
                "extracted dir {extracted_dir:?} does not match cef_dir {cef_dir:?}",
            ));
        }

        let archive_version = serde_json::to_string_pretty(version.minimal()?)?;
        let mut archive_json = File::create(extracted_dir.join("archive.json"))?;
        archive_json.write_all(archive_version.as_bytes())?;
    }

    copy_directory(cef_dir, &output)?;

    Ok(())
}

fn copy_directory<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(&src)? {
        let entry = entry?;
        let dst_path = dst.as_ref().join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory(&entry.path(), &dst_path)?;
        } else {
            fs::copy(&entry.path(), &dst_path)?;
        }
    }
    Ok(())
}
