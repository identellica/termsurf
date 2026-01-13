use cef::build_util::mac::*;
use clap::Parser;
use semver::Version;
use std::{env, path::PathBuf};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    name: String,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(short, long)]
    identifier: Option<String>,
    #[arg(short, long)]
    display_name: Option<String>,
    #[arg(short, long, default_value = "English")]
    region: String,
    #[arg(short, long, default_value = "1.0.0")]
    version: String,
}

pub fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let output = match args.output {
        Some(output) => PathBuf::from(output),
        None => env::current_dir()?,
    };
    let identifier = args
        .identifier
        .unwrap_or_else(|| format!("apps.tauri.cef-rs.{}", args.name));
    let display_name = args.display_name.unwrap_or_else(|| args.name.clone());
    let version = Version::parse(&args.version)?;

    let bundle_info = BundleInfo {
        name: args.name.clone(),
        identifier,
        display_name,
        development_region: args.region,
        version,
    };

    let bundle_path = build_bundle(output.as_path(), &args.name, bundle_info)?;
    let bundle_path = bundle_path.display();
    println!("Run the app from {bundle_path}");
    Ok(())
}
