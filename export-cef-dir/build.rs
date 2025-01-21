use std::{env, fs::File, io::Write, path::PathBuf};

fn main() -> anyhow::Result<()> {
    let cef_dir = env::var("DEP_CEF_DLL_WRAPPER_CEF_DIR")?;
    eprintln!("DEP_CEF_DLL_WRAPPER_CEF_DIR: {cef_dir}");
    let cef_dir_rs = PathBuf::from(env::var("OUT_DIR")?).join("cef_dir.rs");
    let mut cef_dir_rs = File::create(cef_dir_rs)?;
    writeln!(&mut cef_dir_rs, r#"const CEF_DIR: &str = r"{cef_dir}";"#)?;
    Ok(())
}
