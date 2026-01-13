use cef::*;
use std::{
    fmt::Debug,
    sync::OnceLock,
    time::{Duration, Instant},
};

pub const TEST_SEND_PROCESS_MESSAGE: &[u8] = b"testSendProcessMessage";
pub const TEST_SEND_SMR_PROCESS_MESSAGE: &[u8] = b"testSendSMRProcessMessage";

#[derive(Debug)]
pub struct MessageId {
    pub id: u32,
}

impl From<u32> for MessageId {
    fn from(id: u32) -> Self {
        Self { id }
    }
}

impl From<&[u8]> for MessageId {
    fn from(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 4);
        let mut id = [0; 4];
        id.copy_from_slice(bytes);
        Self::from(u32::from_ne_bytes(id))
    }
}

impl From<&MessageId> for [u8; 4] {
    fn from(id: &MessageId) -> Self {
        id.id.to_ne_bytes()
    }
}

pub struct ElapsedMicros {
    duration: u128,
}

impl ElapsedMicros {
    pub fn now() -> Self {
        static START_TIME: OnceLock<Instant> = OnceLock::new();
        let start_time = START_TIME.get_or_init(Instant::now);

        Self {
            duration: start_time.elapsed().as_micros(),
        }
    }
}

impl From<&[u8]> for ElapsedMicros {
    fn from(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 16);
        let mut duration = [0; 16];
        duration.copy_from_slice(bytes);
        Self {
            duration: u128::from_ne_bytes(duration),
        }
    }
}

impl From<&ElapsedMicros> for [u8; 16] {
    fn from(elapsed: &ElapsedMicros) -> Self {
        elapsed.duration.to_ne_bytes()
    }
}

impl Debug for ElapsedMicros {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let duration = u64::try_from(self.duration).unwrap_or(u64::MAX);
        let duration = Duration::from_micros(duration);
        write!(f, "{duration:?}")
    }
}

#[derive(Debug)]
pub struct BrowserMessage {
    pub test_id: MessageId,
    pub start_time: ElapsedMicros,
}

impl From<&BinaryValue> for BrowserMessage {
    fn from(message: &BinaryValue) -> Self {
        assert_eq!(message.size(), 20);

        let mut data = vec![0; message.size()];
        message.data(Some(&mut data), 0);

        Self {
            test_id: MessageId::from(&data[0..4]),
            start_time: ElapsedMicros::from(&data[4..20]),
        }
    }
}

impl From<&BrowserMessage> for Option<BinaryValue> {
    fn from(message: &BrowserMessage) -> Self {
        let mut data = vec![0; 20];

        let test_id: [u8; 4] = (&message.test_id).into();
        let start_time: [u8; 16] = (&message.start_time).into();

        data[0..4].copy_from_slice(&test_id);
        data[4..20].copy_from_slice(&start_time);

        binary_value_create(Some(&data))
    }
}

#[derive(Debug)]
pub struct RendererMessage {
    pub test_id: MessageId,
    pub duration: ElapsedMicros,
    pub start_time: ElapsedMicros,
}

impl From<&BinaryValue> for RendererMessage {
    fn from(message: &BinaryValue) -> Self {
        assert_eq!(message.size(), 36);

        let mut data = vec![0; message.size()];
        message.data(Some(&mut data), 0);

        Self {
            test_id: MessageId::from(&data[0..4]),
            duration: ElapsedMicros::from(&data[4..20]),
            start_time: ElapsedMicros::from(&data[20..36]),
        }
    }
}

impl From<&RendererMessage> for Option<BinaryValue> {
    fn from(message: &RendererMessage) -> Self {
        let mut data = vec![0; 36];

        let test_id: [u8; 4] = (&message.test_id).into();
        let duration: [u8; 16] = (&message.duration).into();
        let start_time: [u8; 16] = (&message.start_time).into();

        data[0..4].copy_from_slice(&test_id);
        data[4..20].copy_from_slice(&duration);
        data[20..36].copy_from_slice(&start_time);

        binary_value_create(Some(&data))
    }
}
