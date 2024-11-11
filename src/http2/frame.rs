use std::fmt::Display;

use crate::{u24::u24, u31::u31};

use super::{payload::Payload, FromBytesError, Len};

pub enum FrameParseError {
    InsufficentLength,
    InsufficentPayloadLength,
    PayloadParseError(FromBytesError),
}

impl From<FromBytesError> for FrameParseError {
    fn from(value: FromBytesError) -> Self {
        FrameParseError::PayloadParseError(value)
    }
}

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
    pub length: u24,
    pub frame_type: FrameType,
    pub flags: u8,
    pub reserved: bool,
    pub stream_id: u31,
    pub payload: Payload,
}

impl Display for FrameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            FrameType::Data => "FrameType::Data",
            FrameType::Headers => "FrameType::Headers",
            FrameType::Priority => "FrameType::Priority",
            FrameType::RstStream => "FrameType::RstStream",
            FrameType::Settings => "FrameType::Settings",
            FrameType::PushPromise => "FrameType::PushPromise",
            FrameType::Ping => "FrameType::Ping",
            FrameType::GoAway => "FrameType::GoAway",
            FrameType::WindowUpdate => "FrameType::WindowUpdate",
            FrameType::Continuation => "FrameType::Continuation",
            FrameType::Unknown => "FrameType::Unknown",
        };
        f.write_str(result)
    }
}

impl Into<u8> for FrameType {
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

impl Into<Vec<u8>> for Frame {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.length.to_bytes());
        result.push(self.frame_type.into());
        result.push(self.flags);
        result.extend((self.stream_id.to_u32() | ((self.reserved as u32) << 31)).to_be_bytes());
        result.extend(<Payload as Into<Vec<u8>>>::into(self.payload));
        result
    }
}

impl From<u8> for FrameType {
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
        let length: [u8; 3] = value[0..3].try_into().unwrap();
        let length = u24::from_bytes(length);

        let frame_type = FrameType::from(value[3]);
        let flags = value[4];

        let stream_id: [u8; 4] = value[5..9].try_into().unwrap();
        let reserved = (u32::from_be_bytes(stream_id) & 0x80000000) == 0x80000000;
        let stream_id = u31::from_bytes(stream_id);

        let payload = Payload::from(
            value[9..(9 + length.to_u32() as usize)].to_vec(),
            flags,
            frame_type.clone(),
        )
        .unwrap();

        Self {
            length,
            frame_type,
            flags,
            reserved,
            stream_id,
            payload,
        }
    }
}

impl Clone for FrameType {
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

impl Len for Frame {
    fn binary_len(&self) -> usize {
        9 + self.payload.binary_len()
    }
}

impl TryFrom<&[u8]> for Frame {
    type Error = FrameParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 9 {
            return Err(FrameParseError::InsufficentLength); // Example error handling
        }

        let length: [u8; 3] = value[0..3].try_into().unwrap();
        let length = u24::from_bytes(length);

        let frame_type = FrameType::from(value[3]);
        let flags = value[4];

        let stream_id: [u8; 4] = value[5..9].try_into().unwrap();
        let reserved = (u32::from_be_bytes(stream_id) & 0x80000000) == 0x80000000;
        let stream_id = u31::from_bytes(stream_id);

        let payload_start = 9;
        let payload_end = payload_start + length.to_u32() as usize;

        if value.len() < payload_end {
            return Err(FrameParseError::InsufficentPayloadLength);
        }

        let payload_data = value[payload_start..payload_end].to_vec();
        let payload = Payload::from(payload_data, flags, frame_type.clone())?;

        Ok(Self {
            length,
            frame_type,
            flags,
            reserved,
            stream_id,
            payload,
        })
    }
}
