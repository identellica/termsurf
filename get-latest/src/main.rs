#![doc = include_str!("../README.md")]

#[macro_use]
extern crate thiserror;

use clap::Parser;
use download_cef::{CefIndex, Channel, LINUX_TARGETS, MACOS_TARGETS, WINDOWS_TARGETS};
use regex::Regex;
use semver::{BuildMetadata, Version};
use std::{fs, path::PathBuf};
use toml_edit::{value, DocumentMut};

#[derive(Debug, Error)]
enum Error {
    #[error("Download error: {0}")]
    Download(#[from] download_cef::Error),
    #[error("Invalid regex pattern: {0}")]
    InvalidRegexPattern(#[from] regex::Error),
    #[error("Invalid version: {0}")]
    InvalidVersion(#[from] semver::Error),
    #[error("No versions found")]
    NoVersionsFound,
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid manifest file: {0}")]
    InvalidManifest(#[from] toml_edit::TomlError),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "stable")]
    channel: Channel,
    #[arg(short, long)]
    update_version: bool,
}

fn main() -> Result<()> {
    let pattern = Regex::new(r"^([^+]+)(:?\+.+)?$")?;

    let args = Args::parse();
    let channel = args.channel;

    let index = CefIndex::download()?;
    let latest_versions: Vec<_> = LINUX_TARGETS
        .iter()
        .chain(MACOS_TARGETS.iter())
        .chain(WINDOWS_TARGETS.iter())
        .map(|target| {
            index
                .platform(target)
                .and_then(|platform| platform.latest(channel.clone()))
                .map(|version| pattern.replace(&version.cef_version, "$1"))
        })
        .collect::<download_cef::Result<Vec<_>>>()?;
    let latest_versions = latest_versions
        .into_iter()
        .map(|version| Ok(Version::parse(&version)?))
        .collect::<Result<Vec<_>>>()?;
    let latest_version = latest_versions
        .into_iter()
        .min()
        .ok_or(Error::NoVersionsFound)?;

    println!("Latest available {channel} version: {latest_version}");

    if args.update_version {
        let current_version =
            Version::parse(&download_cef::default_version(env!("CARGO_PKG_VERSION")))?;
        if current_version < latest_version {
            let mut workspace_version = Version::parse(env!("CARGO_PKG_VERSION"))?;
            if workspace_version.major < latest_version.major {
                workspace_version.major = latest_version.major;
                workspace_version.minor = 0;
            } else {
                workspace_version.minor += 1;
            }
            workspace_version.patch = 0;
            workspace_version.build = BuildMetadata::new(&latest_version.to_string())?;

            let mut manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            manifest.pop();
            let manifest = manifest.join("Cargo.toml");
            let mut doc = fs::read_to_string(&manifest)?.parse::<DocumentMut>()?;
            doc["workspace"]["package"]["version"] = value(workspace_version.to_string());
            workspace_version.build = BuildMetadata::EMPTY;
            doc["workspace"]["dependencies"]["cef-dll-sys"]["version"] =
                value(workspace_version.to_string());
            fs::write(&manifest, doc.to_string().as_bytes())?;
        }
    }

    Ok(())
}
