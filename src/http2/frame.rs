use std::{clone, result, vec};

use crate::http2::payload;

use super::payload::Payload;

#[derive(Debug)]
pub enum FrameType {
    Data = 0,
    Headers = 1,
    Priority = 2,
    RstStream = 3,
    Settings = 4,
    PushPromise = 5,
    Ping = 6,
    GoAway = 7,
    WindowUpdate = 8,
    Continuation = 9,
    Unknown = 254,
}

#[derive(Debug)]
pub struct Frame {
    length: u32,
    frame_type: FrameType,
    flags: u8,
    stream_id: u32,
    payload: Payload,
}

impl Into<u8> for FrameType{
    fn into(self) -> u8 {
        match self {
            FrameType::Data => 0,
            FrameType::Headers => 1,
            FrameType::Priority => 2,
            FrameType::RstStream => 3,
            FrameType::Settings => 4,
            FrameType::PushPromise => 5,
            FrameType::Ping => 6,
            FrameType::GoAway => 7,
            FrameType::WindowUpdate => 8,
            FrameType::Continuation => 9,
            FrameType::Unknown => 254,
        }
    }
}

impl Into<Vec<u8>> for Frame{
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.length.to_be_bytes());
        result.push(self.frame_type.into());
        result.push(self.flags);
        result.extend((self.stream_id & 0x7FFF_FFFF).to_be_bytes());
        result.extend(<Payload as Into<Vec<u8>>>::into(self.payload));
        result
    }
}

impl From<u8> for FrameType{
    fn from(value: u8) -> Self {
        match value {
            0 => FrameType::Data,
            1 => FrameType::Headers,
            2 => FrameType::Priority,
            3 => FrameType::RstStream,
            4 => FrameType::Settings,
            5 => FrameType::PushPromise,
            6 => FrameType::Ping,
            7 => FrameType::GoAway,
            8 => FrameType::WindowUpdate,
            9 => FrameType::Continuation,
            _ => FrameType::Unknown,
        }
    }
}

impl From<Vec<u8>> for Frame {
    fn from(value: Vec<u8>) -> Self {
        let length: [u8; 4] = value[0..4].try_into().unwrap();
        let length = u32::from_be_bytes(length);

        let frame_type = FrameType::from(value[5]);
        let flags = value[6];

        let stream_id: [u8; 4] = value[7..11].try_into().unwrap();
        let mut stream_id = u32::from_be_bytes(stream_id);
        stream_id = stream_id & 0x7FFF_FFFF;

        let payload = Payload::from(value[11..length as usize].to_vec(), flags, frame_type.clone()).unwrap();

        Self{
            length,
            frame_type,
            flags,
            stream_id,
            payload
        }
    }
}

impl Clone for FrameType{
    fn clone(&self) -> Self {
        match self {
            Self::Data => Self::Data,
            Self::Headers => Self::Headers,
            Self::Priority => Self::Priority,
            Self::RstStream => Self::RstStream,
            Self::Settings => Self::Settings,
            Self::PushPromise => Self::PushPromise,
            Self::Ping => Self::Ping,
            Self::GoAway => Self::GoAway,
            Self::WindowUpdate => Self::WindowUpdate,
            Self::Continuation => Self::Continuation,
            Self::Unknown => Self::Unknown,
        }
    }
}