use super::byte_read_handler::*;
use crate::*;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

pub trait File: Send {
    fn stream_reader(&self) -> Option<StreamReader>;
}

pub type FileMap = BTreeMap<String, Arc<Mutex<dyn File>>>;

struct ZipFile {
    data: Vec<u8>,
}

impl File for ZipFile {
    fn stream_reader(&self) -> Option<StreamReader> {
        let mut handler =
            ByteReadHandler::new(Arc::new(Mutex::new(ByteStream::new(self.data.clone()))));
        stream_reader_create_for_handler(Some(&mut handler))
    }
}

#[derive(Default)]
pub struct ZipArchive {
    contents: Mutex<FileMap>,
}

impl ZipArchive {
    pub fn load(
        &self,
        stream: &mut StreamReader,
        password: &str,
        overwrite_existing: bool,
    ) -> usize {
        let Ok(mut contents) = self.contents.lock() else {
            return 0;
        };
        let Some(reader) = zip_reader_create(Some(stream)) else {
            return 0;
        };
        let password = CefString::from(password);

        let mut count = 0;
        loop {
            let size = reader.file_size();
            if size <= 0 {
                // Skip directories and empty files.
                continue;
            }
            let size = size as usize;

            if reader.open_file(Some(&password)) == 0 {
                break;
            }

            let name = CefString::from(&reader.file_name())
                .to_string()
                .to_lowercase();

            if contents.contains_key(&name) {
                if overwrite_existing {
                    contents.remove(&name);
                } else {
                    // Skip files that already exist.
                    continue;
                }
            }

            let mut data = Vec::with_capacity(size);
            while data.len() < size && reader.eof() == 0 {
                let mut chunk = vec![0; size - data.len()];
                let read = reader.read_file(Some(&mut chunk));
                if read <= 0 {
                    break;
                }
                data.extend_from_slice(&chunk[..read as usize]);
            }

            debug_assert_eq!(data.len(), size);
            reader.close_file();
            count += 1;

            // Add the file to the map.
            contents.insert(name, Arc::new(Mutex::new(ZipFile { data })));

            if reader.move_to_next_file() == 0 {
                break;
            }
        }

        count
    }

    pub fn clear(&self) {
        let Ok(mut contents) = self.contents.lock() else {
            return;
        };
        contents.clear();
    }

    pub fn file_count(&self) -> usize {
        let Ok(contents) = self.contents.lock() else {
            return 0;
        };
        contents.len()
    }

    pub fn file(&self, name: &str) -> Option<Arc<Mutex<dyn File>>> {
        let Ok(contents) = self.contents.lock() else {
            return None;
        };
        contents.get(name).cloned()
    }

    pub fn remove_file(&self, name: &str) -> bool {
        let Ok(mut contents) = self.contents.lock() else {
            return false;
        };
        contents.remove(name).is_some()
    }

    pub fn files(&self) -> FileMap {
        let Ok(contents) = self.contents.lock() else {
            return Default::default();
        };
        contents.clone()
    }
}
