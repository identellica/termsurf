use cef::build_util::linux::*;
use clap::Parser;
use std::{env, path::PathBuf};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    name: String,
    #[arg(long, default_value_t = false)]
    release: bool,
    #[arg(short, long)]
    output: Option<String>,
}

pub fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let output = match args.output {
        Some(output) => PathBuf::from(output),
        None => env::current_dir()?,
    };

    let bundle_path = build_bundle(output.as_path(), &args.name, args.release)?;
    let bundle_path = bundle_path.display();
    println!("Run the app from {bundle_path}");
    Ok(())
}
