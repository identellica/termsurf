use cef::*;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

fn allow_file_io() -> bool {
    currently_on(ThreadId::UI) == 0 && currently_on(ThreadId::IO) == 0
}

/// Reads the file at `path` into `contents` and returns true on success and
/// false on error.  In case of I/O error, `contents` holds the data that could
/// be read from the file before the error occurred.  When the file size exceeds
/// `max_size`, the function returns false with `contents` holding the file
/// truncated to `max_size`. `contents` may be [None], in which case this
/// function is useful for its side effect of priming the disk cache (could be
/// used for unit tests). Calling this function on the browser process UI or IO
/// threads is not allowed.
pub fn read_file_to_buffer<P: AsRef<Path>>(
    path: P,
    contents: &mut Option<Vec<u8>>,
    max_size: usize,
) -> bool {
    if !allow_file_io() {
        return false;
    }

    if let Some(contents) = contents.as_mut() {
        contents.clear();
        contents.reserve(max_size);
    }
    let Ok(mut file) = File::open(path) else {
        return false;
    };

    // Many files supplied in `path` have incorrect size (proc files etc).
    // Hence, the file is read sequentially as opposed to a one-shot read.
    const BUFFER_SIZE: usize = 1 << 16;
    let mut buffer = vec![0; BUFFER_SIZE];
    let mut size = 0;

    while let Ok(read) = file.read(&mut buffer) {
        if read == 0 {
            break;
        }

        if let Some(contents) = contents.as_mut() {
            contents.extend_from_slice(&buffer[..read.min(max_size - size)]);
        }

        size += read;
        if size > max_size {
            return false;
        }
    }

    true
}

/// Writes the given buffer into the file, overwriting any data that was
/// previously there. Returns the number of bytes written, or [None] on error.
/// Calling this function on the browser process UI or IO threads is not allowed.
pub fn write_file<P: AsRef<Path>>(path: P, contents: &[u8]) -> Option<usize> {
    if !allow_file_io() {
        return None;
    }

    let mut file = File::create(path).ok()?;
    let mut size = 0;

    while size < contents.len() {
        let write = file.write(&contents[size..]).unwrap_or(0);
        if write == 0 {
            break;
        }
        size += write;
    }

    Some(size)
}
