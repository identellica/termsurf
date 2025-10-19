use super::message_router::*;
use crate::*;
use std::{
    io::{self, Cursor, Read, Write},
    marker::PhantomData,
    mem,
    rc::Rc,
    slice,
    sync::Arc,
};

const NO_ERROR: i32 = 0;

const CONTEXT_ID: usize = 0;
const REQUEST_ID: usize = 1;
const RENDERER_PAYLOAD: usize = 2;
const IS_SUCCESS: usize = 2;
const BROWSER_PAYLOAD: usize = 3;
const IS_PERSISTENT: usize = 3;

pub trait ProcessMessageBuilder {
    fn build_browser_response(&self, context_id: i32, request_id: i32) -> Option<ProcessMessage>;
    fn build_renderer_message(
        &self,
        context_id: i32,
        request_id: i32,
        persistent: bool,
    ) -> Option<ProcessMessage>;
}

#[derive(Clone, Default)]
pub enum MessagePayload {
    #[default]
    Empty,
    String(CefStringUtf8),
    Binary(Arc<dyn BinaryBuffer>),
}

impl MessagePayload {
    fn size(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::String(s) => s.as_slice().map(|s| s.len()).unwrap_or(0),
            Self::Binary(b) => b.data().len(),
        }
    }

    fn read_string<R: Read>(f: &mut R) -> io::Result<Self> {
        let mut buffer = vec![];
        f.read_to_end(&mut buffer)?;
        Ok(String::from_utf8(buffer)
            .map(|value| value.as_str().into())
            .unwrap_or(Self::Empty))
    }

    // fn read_binary<R: Read>(f: &mut R) -> io::Result<Self> {
    //     let mut buffer = vec![];
    //     f.read_to_end(&mut buffer)?;
    //     Ok(binary_value_create(Some(&buffer))
    //         .map(|value| Self::Binary(Arc::new(BinaryValueBuffer::new(None, Some(value)))))
    //         .unwrap_or(Self::Empty))
    // }

    fn write<W: Write>(&self, f: &mut W) -> io::Result<()> {
        let buffer = match self {
            Self::String(s) => s.as_slice(),
            Self::Binary(b) if !b.data().is_empty() => Some(b.data()),
            _ => None,
        };
        if let Some(buffer) = buffer {
            f.write_all(buffer)?;
        }
        Ok(())
    }
}

impl From<&CefString> for MessagePayload {
    fn from(s: &CefString) -> Self {
        Self::String(CefStringUtf8::from(s))
    }
}

impl From<&str> for MessagePayload {
    fn from(s: &str) -> Self {
        Self::String(CefStringUtf8::from(s))
    }
}

impl From<Option<&V8Value>> for MessagePayload {
    fn from(v: Option<&V8Value>) -> Self {
        let b = v.and_then(|v| {
            let ptr = v.array_buffer_data();
            let size = v.array_buffer_byte_length();
            if ptr.is_null() || size == 0 {
                return None;
            }

            let data = unsafe { slice::from_raw_parts(ptr as *const u8, size) };
            binary_value_create(Some(data))
        });
        b.map(|value| Self::Binary(Arc::new(BinaryValueBuffer::new(None, Some(value)))))
            .unwrap_or(Self::Empty)
    }
}

impl From<&[u8]> for MessagePayload {
    fn from(data: &[u8]) -> Self {
        if data.is_empty() {
            Self::Empty
        } else {
            Self::Binary(Arc::new(BinaryValueBuffer::new(
                None,
                binary_value_create(Some(data)),
            )))
        }
    }
}

#[derive(Clone, Default)]
pub struct BrowserMessage {
    pub context_id: i32,
    pub request_id: i32,
    pub is_success: bool,
    pub error_code: i32,
    pub payload: MessagePayload,
}

impl From<Option<ProcessMessage>> for BrowserMessage {
    fn from(message: Option<ProcessMessage>) -> Self {
        let Some(message) = message else {
            return Default::default();
        };

        if let Some(args) = message.argument_list() {
            let context_id = args.int(CONTEXT_ID);
            let request_id = args.int(REQUEST_ID);
            let is_success = args.bool(IS_SUCCESS);

            if is_success != 0 {
                debug_assert_eq!(args.size(), 4);
                match args.get_type(BROWSER_PAYLOAD) {
                    ValueType::STRING => Self {
                        context_id,
                        request_id,
                        is_success: true,
                        error_code: NO_ERROR,
                        payload: MessagePayload::String(CefStringUtf8::from(&CefString::from(
                            &args.string(BROWSER_PAYLOAD),
                        ))),
                    },
                    ValueType::BINARY => Self {
                        context_id,
                        request_id,
                        is_success: true,
                        error_code: NO_ERROR,
                        payload: MessagePayload::Binary(Arc::new(BinaryValueBuffer::new(
                            Some(message),
                            args.binary(BROWSER_PAYLOAD),
                        ))),
                    },
                    payload_type => {
                        assert_eq!(payload_type, ValueType::NULL);
                        Self {
                            context_id,
                            request_id,
                            is_success: true,
                            error_code: NO_ERROR,
                            payload: MessagePayload::Binary(Arc::new(EmptyBinaryBuffer)),
                        }
                    }
                }
            } else {
                debug_assert_eq!(args.size(), 5);
                Self {
                    context_id,
                    request_id,
                    is_success: false,
                    error_code: args.int(3),
                    payload: MessagePayload::String(CefStringUtf8::from(&CefString::from(
                        &args.string(4),
                    ))),
                }
            }
        } else if let Some(region) = message
            .shared_memory_region()
            .filter(|region| region.is_valid() != 0)
        {
            debug_assert!(region.size() >= BrowserMessageHeader::SIZE);
            debug_assert!(!region.memory().is_null());
            let buffer =
                unsafe { slice::from_raw_parts(region.memory() as *const u8, region.size()) };
            let header = {
                let mut cursor = Cursor::new(&buffer[..BrowserMessageHeader::SIZE]);
                BrowserMessageHeader::read(&mut cursor).unwrap_or_default()
            };
            if header.is_binary {
                Self {
                    context_id: header.context_id,
                    request_id: header.request_id,
                    is_success: true,
                    error_code: NO_ERROR,
                    payload: MessagePayload::Binary(Arc::new(SharedMemoryRegionBuffer::new(
                        Some(region),
                        BrowserMessageHeader::SIZE,
                    ))),
                }
            } else {
                let mut cursor = Cursor::new(&buffer[BrowserMessageHeader::SIZE..]);
                Self {
                    context_id: header.context_id,
                    request_id: header.request_id,
                    is_success: true,
                    error_code: NO_ERROR,
                    payload: MessagePayload::read_string(&mut cursor).unwrap_or_default(),
                }
            }
        } else {
            Default::default()
        }
    }
}

#[derive(Clone, Default)]
pub struct RenderMessage {
    pub context_id: i32,
    pub request_id: i32,
    pub is_persistent: bool,
    pub payload: MessagePayload,
}

impl From<Option<ProcessMessage>> for RenderMessage {
    fn from(message: Option<ProcessMessage>) -> Self {
        let Some(message) = message else {
            return Default::default();
        };

        if let Some(args) = message.argument_list() {
            debug_assert_eq!(args.size(), 4);
            let context_id = args.int(CONTEXT_ID);
            let request_id = args.int(REQUEST_ID);
            let is_persistent = args.bool(IS_PERSISTENT) != 0;

            match args.get_type(RENDERER_PAYLOAD) {
                ValueType::STRING => Self {
                    context_id,
                    request_id,
                    is_persistent,
                    payload: MessagePayload::String(CefStringUtf8::from(&CefString::from(
                        &args.string(RENDERER_PAYLOAD),
                    ))),
                },
                ValueType::BINARY => Self {
                    context_id,
                    request_id,
                    is_persistent,
                    payload: MessagePayload::Binary(Arc::new(BinaryValueBuffer::new(
                        Some(message),
                        args.binary(RENDERER_PAYLOAD),
                    ))),
                },
                payload_type => {
                    debug_assert_eq!(payload_type, ValueType::NULL);
                    Self {
                        context_id,
                        request_id,
                        is_persistent,
                        payload: MessagePayload::Binary(Arc::new(EmptyBinaryBuffer)),
                    }
                }
            }
        } else if let Some(region) = message
            .shared_memory_region()
            .filter(|region| region.is_valid() != 0)
        {
            debug_assert!(region.size() >= RendererMessageHeader::<true>::SIZE);
            debug_assert!(!region.memory().is_null());
            let buffer =
                unsafe { slice::from_raw_parts(region.memory() as *const u8, region.size()) };
            let header = {
                let mut cursor = Cursor::new(&buffer[..RendererMessageHeader::<true>::SIZE]);
                RendererMessageHeader::<true>::read(&mut cursor).unwrap_or_default()
            };
            if header.is_binary {
                Self {
                    context_id: header.context_id,
                    request_id: header.request_id,
                    is_persistent: header.is_persistent,
                    payload: MessagePayload::Binary(Arc::new(SharedMemoryRegionBuffer::new(
                        Some(region),
                        RendererMessageHeader::<true>::SIZE,
                    ))),
                }
            } else {
                let mut cursor = Cursor::new(&buffer[RendererMessageHeader::<true>::SIZE..]);
                Self {
                    context_id: header.context_id,
                    request_id: header.request_id,
                    is_persistent: header.is_persistent,
                    payload: MessagePayload::read_string(&mut cursor).unwrap_or_default(),
                }
            }
        } else {
            Default::default()
        }
    }
}

#[cfg(not(feature = "sandbox"))]
wrap_v8_array_buffer_release_callback! {
    pub struct BinaryValueArrayBufferReleaseCallback {
        value: MessagePayload,
    }

    impl V8ArrayBufferReleaseCallback {
        fn release_buffer(&self, _buffer: *mut u8) {}
    }
}

trait MessageHeader: Sized {
    const SIZE: usize;

    fn new(context_id: i32, request_id: i32, is_binary: bool) -> Self;
    fn read<R: Read>(f: &mut R) -> io::Result<Self>;
    fn write<W: Write>(&self, f: &mut W) -> io::Result<()>;
}

#[derive(Clone, Default)]
struct BrowserMessageHeader {
    context_id: i32,
    request_id: i32,
    is_binary: bool,
}

impl MessageHeader for BrowserMessageHeader {
    const SIZE: usize = mem::size_of::<i32>() * 2 + 1;

    fn new(context_id: i32, request_id: i32, is_binary: bool) -> Self {
        Self {
            context_id,
            request_id,
            is_binary,
        }
    }

    fn read<R: Read>(f: &mut R) -> io::Result<Self> {
        let mut data = [0_u8; mem::size_of::<i32>()];
        f.read_exact(&mut data)?;
        let context_id = i32::from_ne_bytes(data);
        f.read_exact(&mut data)?;
        let request_id = i32::from_ne_bytes(data);
        let mut flags = [0_u8];
        f.read_exact(&mut flags)?;
        let is_binary = flags[0] != 0;

        Ok(Self {
            context_id,
            request_id,
            is_binary,
        })
    }

    fn write<W: Write>(&self, f: &mut W) -> io::Result<()> {
        f.write_all(&self.context_id.to_ne_bytes())?;
        f.write_all(&self.request_id.to_ne_bytes())?;
        f.write_all(&[self.is_binary.into()])
    }
}

#[derive(Clone, Default)]
struct RendererMessageHeader<const DEFAULT_PERSISTENT: bool> {
    context_id: i32,
    request_id: i32,
    is_persistent: bool,
    is_binary: bool,
}

impl<const DEFAULT_PERSISTENT: bool> MessageHeader for RendererMessageHeader<DEFAULT_PERSISTENT> {
    const SIZE: usize = mem::size_of::<i32>() * 2 + 2;

    fn new(context_id: i32, request_id: i32, is_binary: bool) -> Self {
        Self {
            context_id,
            request_id,
            is_persistent: DEFAULT_PERSISTENT,
            is_binary,
        }
    }

    fn read<R: Read>(f: &mut R) -> io::Result<Self> {
        let mut data = [0_u8; mem::size_of::<i32>()];
        f.read_exact(&mut data)?;
        let context_id = i32::from_ne_bytes(data);
        f.read_exact(&mut data)?;
        let request_id = i32::from_ne_bytes(data);
        let mut flags = [0_u8; 2];
        f.read_exact(&mut flags)?;
        let is_persistent = flags[0] != 0;
        let is_binary = flags[1] != 0;

        Ok(Self {
            context_id,
            request_id,
            is_persistent,
            is_binary,
        })
    }

    fn write<W: Write>(&self, f: &mut W) -> io::Result<()> {
        f.write_all(&self.context_id.to_ne_bytes())?;
        f.write_all(&self.request_id.to_ne_bytes())?;
        f.write_all(&[self.is_persistent.into()])?;
        f.write_all(&[self.is_binary.into()])
    }
}

fn build_browser_list_message(
    name: &str,
    context_id: i32,
    request_id: i32,
    payload: MessagePayload,
) -> Option<ProcessMessage> {
    let message = process_message_create(Some(&CefString::from(name)))?;
    let args = message.argument_list()?;
    args.set_int(CONTEXT_ID, context_id);
    args.set_int(REQUEST_ID, request_id);
    args.set_bool(IS_SUCCESS, 1);

    match payload {
        MessagePayload::Empty => args.set_null(BROWSER_PAYLOAD),
        MessagePayload::String(value) => {
            args.set_string(BROWSER_PAYLOAD, Some(&CefString::from(&value)))
        }
        MessagePayload::Binary(value) => args.set_binary(
            BROWSER_PAYLOAD,
            binary_value_create(Some(value.data())).as_mut(),
        ),
    };

    Some(message)
}

fn build_render_list_message(
    name: &str,
    context_id: i32,
    request_id: i32,
    payload: MessagePayload,
    is_persistent: bool,
) -> Option<ProcessMessage> {
    let message = process_message_create(Some(&CefString::from(name)))?;
    let args = message.argument_list()?;
    args.set_int(CONTEXT_ID, context_id);
    args.set_int(REQUEST_ID, request_id);

    match payload {
        MessagePayload::Empty => args.set_null(RENDERER_PAYLOAD),
        MessagePayload::String(value) => {
            args.set_string(RENDERER_PAYLOAD, Some(&CefString::from(&value)))
        }
        MessagePayload::Binary(value) => args.set_binary(
            RENDERER_PAYLOAD,
            binary_value_create(Some(value.data())).as_mut(),
        ),
    };

    args.set_bool(IS_PERSISTENT, is_persistent.into());

    Some(message)
}

struct MessagePayloadBuilder {
    pub name: String,
    pub payload: MessagePayload,
}

impl ProcessMessageBuilder for MessagePayloadBuilder {
    fn build_browser_response(&self, context_id: i32, request_id: i32) -> Option<ProcessMessage> {
        build_browser_list_message(&self.name, context_id, request_id, self.payload.clone())
    }

    fn build_renderer_message(
        &self,
        context_id: i32,
        request_id: i32,
        persistent: bool,
    ) -> Option<ProcessMessage> {
        build_render_list_message(
            &self.name,
            context_id,
            request_id,
            self.payload.clone(),
            persistent,
        )
    }
}

enum SharedProcessMessageRouter<Header>
where
    Header: MessageHeader,
{
    SharedMemory {
        builder: SharedProcessMessageBuilder,
        is_binary: bool,
        _phantom: PhantomData<Header>,
    },
    Payload(MessagePayloadBuilder),
}

impl<Header> SharedProcessMessageRouter<Header>
where
    Header: MessageHeader,
{
    fn new(name: &str, payload: MessagePayload) -> Self {
        let message_size = Header::SIZE + payload.size();
        let builder =
            match shared_process_message_builder_create(Some(&CefString::from(name)), message_size)
            {
                Some(builder) if builder.is_valid() != 0 => builder,
                _ => {
                    return Self::Payload(MessagePayloadBuilder {
                        name: name.to_owned(),
                        payload,
                    })
                }
            };

        let buffer =
            unsafe { slice::from_raw_parts_mut(builder.memory() as *mut u8, message_size) };
        let mut cursor = Cursor::new(&mut buffer[Header::SIZE..]);
        if payload.write(&mut cursor).is_err() {
            return Self::Payload(MessagePayloadBuilder {
                name: name.to_owned(),
                payload,
            });
        }

        Self::SharedMemory {
            builder,
            is_binary: matches!(payload, MessagePayload::Binary(_)),
            _phantom: PhantomData,
        }
    }
}

impl<Header> ProcessMessageBuilder for SharedProcessMessageRouter<Header>
where
    Header: MessageHeader,
{
    fn build_browser_response(&self, context_id: i32, request_id: i32) -> Option<ProcessMessage> {
        match self {
            Self::SharedMemory {
                builder, is_binary, ..
            } => {
                let buffer =
                    unsafe { slice::from_raw_parts_mut(builder.memory() as *mut u8, Header::SIZE) };
                let mut cursor = Cursor::new(buffer);
                Header::new(context_id, request_id, *is_binary)
                    .write(&mut cursor)
                    .ok()?;

                builder.build()
            }
            Self::Payload(builder) => builder.build_browser_response(context_id, request_id),
        }
    }

    fn build_renderer_message(
        &self,
        context_id: i32,
        request_id: i32,
        persistent: bool,
    ) -> Option<ProcessMessage> {
        match self {
            Self::SharedMemory {
                builder, is_binary, ..
            } => {
                let buffer =
                    unsafe { slice::from_raw_parts_mut(builder.memory() as *mut u8, Header::SIZE) };
                let mut cursor = Cursor::new(buffer);
                Header::new(context_id, request_id, *is_binary)
                    .write(&mut cursor)
                    .ok()?;

                builder.build()
            }
            Self::Payload(builder) => {
                builder.build_renderer_message(context_id, request_id, persistent)
            }
        }
    }
}

pub struct EmptyBinaryBuffer;

impl BinaryBuffer for EmptyBinaryBuffer {
    fn data(&self) -> &[u8] {
        &[]
    }

    fn data_mut(&mut self) -> &mut [u8] {
        &mut []
    }
}

pub struct BinaryValueBuffer {
    _message: Option<ProcessMessage>,
    value: Option<BinaryValue>,
}

impl BinaryValueBuffer {
    pub fn new(message: Option<ProcessMessage>, value: Option<BinaryValue>) -> Self {
        Self {
            _message: message,
            value,
        }
    }
}

impl BinaryBuffer for BinaryValueBuffer {
    fn data(&self) -> &[u8] {
        self.value.as_ref().map_or(&[], |v| unsafe {
            slice::from_raw_parts(v.raw_data() as *const u8, v.size())
        })
    }

    fn data_mut(&mut self) -> &mut [u8] {
        self.value.as_mut().map_or(&mut [], |v| unsafe {
            slice::from_raw_parts_mut(v.raw_data() as *mut u8, v.size())
        })
    }
}

struct SharedMemoryRegionBuffer {
    region: Option<SharedMemoryRegion>,
    offset: usize,
}

impl SharedMemoryRegionBuffer {
    fn new(region: Option<SharedMemoryRegion>, offset: usize) -> Self {
        Self { region, offset }
    }

    fn data(&self) -> *mut u8 {
        self.region
            .as_ref()
            .map_or(std::ptr::null_mut(), |r| unsafe {
                r.memory().add(self.offset) as *mut u8
            })
    }

    fn size(&self) -> usize {
        self.region
            .as_ref()
            .map_or(0, |r| r.size().saturating_sub(self.offset))
    }
}

impl BinaryBuffer for SharedMemoryRegionBuffer {
    fn data(&self) -> &[u8] {
        let data = self.data();
        let size = self.size();
        if data.is_null() || size == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(data as *const u8, size) }
        }
    }

    fn data_mut(&mut self) -> &mut [u8] {
        let data = self.data();
        let size = self.size();
        if data.is_null() || size == 0 {
            &mut []
        } else {
            unsafe { slice::from_raw_parts_mut(data, size) }
        }
    }
}

pub fn create_browser_response_builder(
    threshold: usize,
    name: &str,
    payload: MessagePayload,
) -> Rc<dyn ProcessMessageBuilder> {
    if payload.size() < threshold {
        Rc::new(MessagePayloadBuilder {
            name: name.to_string(),
            payload,
        })
    } else {
        Rc::new(SharedProcessMessageRouter::<BrowserMessageHeader>::new(
            name, payload,
        ))
    }
}

pub fn build_renderer_message(
    threshold: usize,
    name: &str,
    context_id: i32,
    request_id: i32,
    request: Option<&V8Value>,
    persistent: bool,
) -> Option<ProcessMessage> {
    let payload = request
        .map(|request| {
            if request.is_string() != 0 {
                MessagePayload::from(&CefString::from(&request.string_value()))
            } else {
                MessagePayload::from(Some(request))
            }
        })
        .unwrap_or(MessagePayload::Empty);

    let builder: Box<dyn ProcessMessageBuilder> = if payload.size() < threshold {
        Box::new(MessagePayloadBuilder {
            name: name.to_string(),
            payload,
        })
    } else if persistent {
        Box::new(SharedProcessMessageRouter::<RendererMessageHeader<true>>::new(name, payload))
    } else {
        Box::new(SharedProcessMessageRouter::<RendererMessageHeader<false>>::new(name, payload))
    };

    builder.build_renderer_message(context_id, request_id, persistent)
}
