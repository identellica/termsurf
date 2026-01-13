//! Implementation of the ResourceHandler class for reading from a Stream.
use crate::*;

wrap_resource_handler! {
    pub struct StreamResourceHandler {
        status_code: i32,
        status_text: String,
        mime_type: String,
        header_map: Option<CefStringMultimap>,
        stream: Option<StreamReader>,
    }

    impl ResourceHandler {
        fn open(
            &self,
            _request: Option<&mut Request>,
            handle_request: Option<&mut i32>,
            _callback: Option<&mut Callback>,
        ) -> i32 {
            debug_assert_eq!(
                currently_on(ThreadId::UI),
                0,
                "open must not be called on the UI thread"
            );
            debug_assert_eq!(
                currently_on(ThreadId::IO),
                0,
                "open must not be called on the IO thread"
            );

            // Continue the request immediately.
            if let Some(handle_request) = handle_request {
                *handle_request = 1;
            }

            1
        }

        fn response_headers(
            &self,
            response: Option<&mut Response>,
            response_length: Option<&mut i64>,
            _redirect_url: Option<&mut CefString>,
        ) {
            debug_assert_ne!(
                currently_on(ThreadId::IO),
                0,
                "response_headers must be called on the IO thread"
            );

            let Some(response) = response else {
                return;
            };

            response.set_status(self.status_code);
            response.set_status_text(Some(&CefString::from(self.status_text.as_str())));
            response.set_mime_type(Some(&CefString::from(self.mime_type.as_str())));

            if let Some(mut header_map) = self.header_map.clone() {
                response.set_header_map(Some(&mut header_map));
            }

            if let Some(response_length) = response_length {
                *response_length = if self.stream.is_some() { -1 } else { 0 };
            }
        }

        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        fn read(
            &self,
            data_out: *mut u8,
            bytes_to_read: i32,
            bytes_read: Option<&mut i32>,
            _callback: Option<&mut ResourceReadCallback>,
        ) -> i32 {
            debug_assert_eq!(
                currently_on(ThreadId::UI),
                0,
                "read must not be called on the UI thread"
            );
            debug_assert_eq!(
                currently_on(ThreadId::IO),
                0,
                "read must not be called on the IO thread"
            );
            if bytes_to_read < 1 {
                return 0;
            }
            let Some(bytes_read) = bytes_read else {
                return 0;
            };
            let Some(stream) = &self.stream else {
                *bytes_read = 0;
                return 1;
            };

            // Read until the buffer is full or until read() returns 0 to indicate no more data.
            *bytes_read = 0;
            loop {
                let data_out = unsafe { data_out.add(*bytes_read as usize) };
                let read = stream.read(data_out, 1, (bytes_to_read - *bytes_read) as usize);
                *bytes_read += read as i32;
                if read == 0 || *bytes_read >= bytes_to_read {
                    break;
                }
            }

            if *bytes_read > 0 {
                1
            } else {
                0
            }
        }
    }
}

impl StreamResourceHandler {
    pub fn new_with_stream(mime_type: String, stream: StreamReader) -> ResourceHandler {
        Self::new(200, "OK".to_string(), mime_type, None, Some(stream))
    }
}
