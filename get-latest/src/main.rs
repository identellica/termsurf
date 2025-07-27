#![doc = include_str!("../README.md")]

#[macro_use]
extern crate thiserror;

use clap::Parser;
use download_cef::{CefIndex, Channel, LINUX_TARGETS, MACOS_TARGETS, WINDOWS_TARGETS};
use git_cliff::args::*;
use git_cliff_core::config::BumpType;
use regex::Regex;
use semver::{BuildMetadata, Version};
use std::{env, fs, io::Write, path::PathBuf};
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
    #[error("Error running git-cliff: {0:?}")]
    InvalidGitCliffArgs(#[from] clap::Error),
    #[error("Error updating change log: {0:?}")]
    UpdateChangeLog(#[from] git_cliff_core::error::Error),
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
            let latest_build = BuildMetadata::new(&latest_version.to_string())?;
            let mut next_version = Version::parse(env!("CARGO_PKG_VERSION"))?;
            if next_version.major < latest_version.major {
                next_version.major = latest_version.major;
                next_version.minor = 0;
            } else {
                next_version.minor += 1;
            }
            next_version.patch = 0;
            next_version.build = latest_build.clone();

            let mut manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            manifest.pop();
            let manifest = manifest.join("Cargo.toml");
            let mut doc = fs::read_to_string(&manifest)?.parse::<DocumentMut>()?;
            doc["workspace"]["package"]["version"] = value(next_version.to_string());
            let workspace_version = Version {
                build: BuildMetadata::EMPTY,
                ..next_version.clone()
            };
            doc["workspace"]["dependencies"]["cef-dll-sys"]["version"] =
                value(workspace_version.to_string());
            fs::write(&manifest, doc.to_string().as_bytes())?;

            if let Ok(output) = env::var("GITHUB_OUTPUT") {
                let mut output = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(output)?;

                let commit_message = format!("chore: update CEF version to {latest_version}");
                writeln!(output, "commit-message={commit_message}",)?;

                let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                config_path.push("cliff.toml");

                let mut tag_version = Version {
                    build: latest_build,
                    ..Version::parse(env!("CARGO_PKG_VERSION"))?
                };

                let bump = Some(BumpOption::Specific(
                    if tag_version.major < latest_version.major {
                        tag_version.major = latest_version.major - 1;
                        BumpType::Major
                    } else {
                        BumpType::Minor
                    },
                ));

                let common_opts = ["--strip", "footer", "--include-path", "Cargo.toml"];

                let export_cef_dir_tag = format!("export-cef-dir-v{tag_version}");
                let export_cef_dir_opts = Opt {
                    config: config_path.clone(),
                    with_commit: Some(vec![commit_message.clone()]),
                    bump: bump.clone(),
                    range: Some("export-cef-dir-v138.2.0+138.0.21..".to_string()),
                    ..Opt::try_parse_from(
                        common_opts.iter().chain(
                            [
                                "--include-path",
                                "export-cef-dir/*",
                                "--tag-pattern",
                                "^export-cef-dir-v",
                                "--tag",
                                export_cef_dir_tag.as_str(),
                                "--output",
                                "export-cef-dir/CHANGELOG.md",
                            ]
                            .iter(),
                        ),
                    )?
                };
                git_cliff::run(export_cef_dir_opts)?;

                let cef_dll_sys_tag = format!("cef-dll-sys-v{tag_version}");
                let cef_dll_sys_opts = Opt {
                    config: config_path.clone(),
                    with_commit: Some(vec![commit_message.clone()]),
                    bump: bump.clone(),
                    range: Some("cef-dll-sys-v138.2.0+138.0.21..".to_string()),
                    ..Opt::try_parse_from(
                        common_opts.iter().chain(
                            [
                                "--include-path",
                                "sys/*",
                                "--tag-pattern",
                                "^cef-dll-sys-v",
                                "--tag",
                                cef_dll_sys_tag.as_str(),
                                "--output",
                                "sys/CHANGELOG.md",
                            ]
                            .iter(),
                        ),
                    )?
                };
                git_cliff::run(cef_dll_sys_opts)?;

                let cef_tag = format!("cef-v{tag_version}");
                let cef_opts = Opt {
                    config: config_path,
                    with_commit: Some(vec![commit_message]),
                    bump,
                    range: Some("cef-v138.2.0+138.0.21..".to_string()),
                    ..Opt::try_parse_from(
                        common_opts.iter().chain(
                            [
                                "--include-path",
                                "cef/*",
                                "--tag-pattern",
                                "^cef-v",
                                "--tag",
                                cef_tag.as_str(),
                                "--output",
                                "cef/CHANGELOG.md",
                            ]
                            .iter(),
                        ),
                    )?
                };
                git_cliff::run(cef_opts)?;
            }
        }
    }

    Ok(())
}
