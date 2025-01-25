use clap::Parser;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

include!(concat!(env!("OUT_DIR"), "/cef_dir.rs"));

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "false")]
    force: bool,
    target: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let target = PathBuf::from(args.target);

    if target.exists() {
        if !args.force {
            return Err(anyhow::anyhow!(
                "target directory already exists: {}",
                target.display()
            ));
        }

        let parent = target.parent();
        let dir = target
            .file_name()
            .and_then(|dir| dir.to_str())
            .ok_or_else(|| anyhow::anyhow!("invalid target directory: {}", target.display()))?;
        let old_target = parent.map(|p| p.join(format!("old_{dir}")));
        let old_target = old_target.as_ref().unwrap_or(&target);
        fs::rename(&target, old_target)?;
        println!("Cleaning up: {}", old_target.display());
        fs::remove_dir_all(old_target)?
    }

    copy_directory(PathBuf::from(CEF_DIR), &target)?;

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
