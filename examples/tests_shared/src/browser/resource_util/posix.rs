use cef::*;
use std::{fs::File, io::Read, path::PathBuf};

pub fn get_resource_directory() -> Option<PathBuf> {
    let mut path = std::env::current_exe().ok()?;

    // Pop the executable file name.
    path.pop();

    #[cfg(target_os = "macos")]
    {
        // Pop the MacOS directory.
        path.pop();
        path.push("Resources");
    }

    #[cfg(target_os = "linux")]
    {
        path.push("files");
    }

    Some(path)
}

pub fn load_binary_resource(resource_name: &str) -> Option<Vec<u8>> {
    let path = get_resource_directory()?.join(resource_name);
    let mut file = File::open(path).ok()?;
    let mut data = Vec::new();
    file.read_to_end(&mut data).ok()?;
    Some(data)
}

pub fn get_binary_resource_reader(resource_name: &str) -> Option<StreamReader> {
    let path = get_resource_directory()?.join(resource_name);
    if !path.exists() {
        return None;
    }

    let path = path.to_str()?;
    let path = CefString::from(path);
    stream_reader_create_for_file(Some(&CefString::from(path)))
}
