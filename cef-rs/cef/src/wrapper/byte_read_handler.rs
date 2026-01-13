//! Thread safe implementation of the [ReadHandler] type for reading an in-memory array of bytes.
use crate::*;
use std::sync::{Arc, Mutex};

pub struct ByteStream {
    bytes: Vec<u8>,
    offset: usize,
}

impl ByteStream {
    pub fn new(bytes: Vec<u8>) -> Self {
        ByteStream { bytes, offset: 0 }
    }
}

wrap_read_handler! {
    pub struct ByteReadHandler {
        stream: Arc<Mutex<ByteStream>>,
    }

    impl ReadHandler {
        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        fn read(&self, ptr: *mut u8, size: usize, n: usize) -> usize {
            let Ok(mut stream) = self.stream.lock() else {
                return 0;
            };

            let s = (stream.bytes.len() - stream.offset) / size;
            let ret = s.min(n);
            let buffer = unsafe { std::slice::from_raw_parts_mut(ptr, ret * size) };
            buffer.copy_from_slice(&stream.bytes[stream.offset..stream.offset + ret * size]);
            stream.offset += ret * size;
            ret
        }

        fn seek(&self, offset: i64, whence: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
            let Ok(mut stream) = self.stream.lock() else {
                return -1;
            };

            const SEEK_SET: i32 = 0;
            const SEEK_CUR: i32 = 1;
            const SEEK_END: i32 = 2;

            match whence {
                SEEK_SET => {
                    if offset < 0 {
                        return -1;
                    }
                    let offset = offset as usize;
                    if offset > stream.bytes.len() {
                        return -1;
                    }
                    stream.offset = offset;
                    0
                }
                SEEK_CUR => {
                    if offset < 0 {
                        let offset = -offset as usize;
                        if offset > stream.offset {
                            return -1;
                        }
                        stream.offset -= offset;
                    } else {
                        let offset = offset as usize;
                        if offset + stream.offset > stream.bytes.len() {
                            return -1;
                        }
                        stream.offset += offset;
                    }
                    0
                }
                SEEK_END => {
                    if offset > 0 {
                        return -1;
                    }
                    let offset = -offset as usize;
                    if offset > stream.bytes.len() {
                        return -1;
                    }
                    stream.offset = stream.bytes.len() - offset;
                    0
                }
                _ => -1,
            }
        }

        fn tell(&self) -> i64 {
            let Ok(stream) = self.stream.lock() else {
                return 0;
            };
            stream.offset as i64
        }

        fn eof(&self) -> i32 {
            let Ok(stream) = self.stream.lock() else {
                return 1;
            };
            if stream.offset >= stream.bytes.len() {
                1
            } else {
                0
            }
        }

        fn may_block(&self) -> i32 {
            0
        }
    }
}
