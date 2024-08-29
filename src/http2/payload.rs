use std::result;

use crate::u31::u31;

use super::{
    frame::FrameType,
    hpack::{self, Hpack},
    len,
    payload_flags::{DataPayloadFlag, HeadersPayloadFlag, PushPromisePayloadFlag},
};

#[derive(Debug)]
pub enum FromBytesError {
    InvalidLength,
    InvalidFlag,
    InvalidPayloadType,
    Utf8Error(std::string::FromUtf8Error),
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
}

pub trait FromBytes<T> {
    fn from(value: Vec<u8>, flags: u8) -> Result<T, FromBytesError>;
}

#[derive(Debug)]
pub struct DataPayload {
    pub PadLength: Option<u8>,
    pub data: Vec<u8>,
    pub Padding: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct HeadersPayload {
    pub PadLength: Option<u8>,
    pub Priority: Option<PriorityPayload>,
    pub HeaderBlockFragment: Hpack,
    pub Padding: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct PriorityPayload {
    pub ExclusiveFlag: bool,
    pub StreamDependency: u31,
    pub Weight: u8,
}

#[derive(Debug)]
pub struct RstStreamPayload {
    pub ErrorCode: u32,
}

type SettingIdentifier = u16;
type SettingValue = u32;
#[derive(Debug)]
pub struct SettingsPayload {
    pub settings: Vec<(SettingIdentifier, SettingValue)>,
}

#[derive(Debug)]
pub struct PushPromisePayload {
    pub PadLength: Option<u8>,
    pub PromisedStreamId: u31,
    pub HeaderBlockFragment: Hpack,
    pub Padding: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct PingPayload {
    pub OpaqueData: u64,
}

#[derive(Debug)]
pub struct GoAwayPayload {
    pub LastStreamId: u31,
    pub ErrorCode: u32,
    pub AdditionalData: Vec<u8>,
}

#[derive(Debug)]
pub struct WindowUpdatePayload {
    pub WindowSizeIncrement: u32, //31 bit intiger
}

#[derive(Debug)]
pub struct ContinuationPayload {
    pub HeaderBlockFragment: Hpack,
}

#[derive(Debug)]
pub enum Payload {
    Data(DataPayload),                 // 0
    Headers(HeadersPayload),           // 1
    Priority(PriorityPayload),         // 2
    RstStream(RstStreamPayload),       // 3
    Settings(SettingsPayload),         // 4
    PushPromise(PushPromisePayload),   // 5
    Ping(PingPayload),                 // 6
    GoAway(GoAwayPayload),             // 7
    WindowUpdate(WindowUpdatePayload), // 8
    Continuation(ContinuationPayload), // 9
}

impl Into<Vec<u8>> for DataPayload {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        if let Some(pad_length) = self.PadLength {
            result.push(pad_length);
        }
        result.extend(self.data);
        if let Some(padding) = self.Padding {
            result.extend(padding);
        }
        result
    }
}

impl Into<Vec<u8>> for PriorityPayload {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        let c = ((self.ExclusiveFlag as u32) << 31) & self.StreamDependency.to_u32();
        result.extend(c.to_be_bytes());
        result.push(self.Weight);
        result
    }
}

impl Into<Vec<u8>> for HeadersPayload {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        if let Some(pad_length) = self.PadLength {
            result.push(pad_length);
        }
        if let Some(priority) = self.Priority {
            result.extend::<Vec<u8>>(priority.into());
        }

        result.extend::<Vec<u8>>(self.HeaderBlockFragment.into());

        if let Some(padding) = self.Padding {
            result.extend(padding);
        }
        result
    }
}

impl Into<Vec<u8>> for RstStreamPayload {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.ErrorCode.to_be_bytes());
        result
    }
}

impl Into<Vec<u8>> for SettingsPayload {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        for (key, value) in self.settings {
            result.extend(key.to_be_bytes());
            result.extend(value.to_be_bytes());
        }
        result
    }
}

impl Into<Vec<u8>> for PushPromisePayload {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        if let Some(pad_length) = self.PadLength {
            result.push(pad_length);
        }

        result.extend(self.PromisedStreamId.to_bytes());
        result.extend::<Vec<u8>>(self.HeaderBlockFragment.into());

        if let Some(padding) = self.Padding {
            result.extend(padding);
        }
        result
    }
}

impl Into<Vec<u8>> for PingPayload {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.OpaqueData.to_be_bytes());
        result
    }
}

impl Into<Vec<u8>> for GoAwayPayload {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.LastStreamId.to_bytes());
        result.extend(self.ErrorCode.to_be_bytes());
        result.extend(self.AdditionalData);
        result
    }
}

impl Into<Vec<u8>> for WindowUpdatePayload {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.WindowSizeIncrement.to_be_bytes());
        result
    }
}

impl Into<Vec<u8>> for ContinuationPayload {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend::<Vec<u8>>(self.HeaderBlockFragment.into());
        result
    }
}

impl Into<Vec<u8>> for Payload {
    fn into(self) -> Vec<u8> {
        match self {
            Payload::Data(v) => v.into(),
            Payload::Headers(v) => v.into(),
            Payload::Priority(v) => v.into(),
            Payload::RstStream(v) => v.into(),
            Payload::Settings(v) => v.into(),
            Payload::PushPromise(v) => v.into(),
            Payload::Ping(v) => v.into(),
            Payload::GoAway(v) => v.into(),
            Payload::WindowUpdate(v) => v.into(),
            Payload::Continuation(v) => v.into(),
        }
    }
}

impl FromBytes<DataPayload> for DataPayload {
    fn from(value: Vec<u8>, flag: u8) -> Result<DataPayload, FromBytesError> {
        if (flag & DataPayloadFlag::PADDED) == DataPayloadFlag::PADDED {
            let result = DataPayload {
                PadLength: Some(value[0]),
                data: value[0..value[0] as usize].to_vec(),
                Padding: Some(value[value[0] as usize..].to_vec()),
            };
            Ok(result)
        } else {
            let result = DataPayload {
                PadLength: None,
                data: value,
                Padding: None,
            };
            Ok(result)
        }
    }
}

impl FromBytes<PriorityPayload> for PriorityPayload {
    fn from(value: Vec<u8>, flag: u8) -> Result<Self, FromBytesError> {
        let b32: [u8; 4] = value[0..4].try_into().unwrap();
        let stream_dependency = u31::from_bytes(b32);
        Ok(PriorityPayload {
            ExclusiveFlag: (stream_dependency.to_u32() & 0x80000000) == 0x80000000,
            StreamDependency: stream_dependency,
            Weight: value[4],
        })
    }
}

impl FromBytes<HeadersPayload> for HeadersPayload {
    fn from(value: Vec<u8>, flag: u8) -> Result<Self, FromBytesError> {
        let mut PadLength = None;
        let mut header_start = 0;
        let mut header_end = value.len();
        if flag & HeadersPayloadFlag::PADDED == HeadersPayloadFlag::PADDED {
            PadLength = Some(value[0]);
            header_start += 1;
            header_end -= PadLength.unwrap() as usize;
        }

        let mut priority: Option<PriorityPayload> = None;
        if flag & HeadersPayloadFlag::PRIORITY == HeadersPayloadFlag::PRIORITY {
            priority = Some(<PriorityPayload as FromBytes<PriorityPayload>>::from(
                value[1..6].to_vec(),
                flag,
            )? as PriorityPayload);
            header_start += 5;
        }

        Ok(HeadersPayload {
            PadLength: PadLength,
            Priority: priority,
            HeaderBlockFragment: <Hpack as From<Vec<u8>>>::from(
                value[header_start..header_end].to_vec(),
            ),
            Padding: Some(value[header_end..].to_vec()),
        })
    }
}

impl FromBytes<RstStreamPayload> for RstStreamPayload {
    fn from(value: Vec<u8>, flag: u8) -> Result<Self, FromBytesError> {
        let b32: [u8; 4] = value[0..4].try_into().unwrap();
        let b32 = u32::from_be_bytes(b32);
        Ok(RstStreamPayload { ErrorCode: b32 })
    }
}

impl FromBytes<SettingsPayload> for SettingsPayload {
    fn from(value: Vec<u8>, flag: u8) -> Result<Self, FromBytesError> {
        let mut result = Vec::new();
        for i in (0..(value.len() / 6)) {
            let b16: [u8; 2] = value[i..(i + 2)].try_into().unwrap();
            let b16 = u16::from_be_bytes(b16);
            let b32: [u8; 4] = value[(i + 2)..(i + 6)].try_into().unwrap();
            let b32 = u32::from_be_bytes(b32);
            result.push((b16, b32));
        }
        Ok(SettingsPayload { settings: result })
    }
}

impl FromBytes<PushPromisePayload> for PushPromisePayload {
    fn from(value: Vec<u8>, flag: u8) -> Result<Self, FromBytesError> {
        let mut PadLength = None;
        let mut stream_id_start = 0;
        let mut header_start = 4;
        let mut header_end = value.len();
        if flag & PushPromisePayloadFlag::PADDED == PushPromisePayloadFlag::PADDED {
            PadLength = Some(value[0]);
            stream_id_start += 1;
            header_start += 1;
            header_end -= PadLength.unwrap() as usize;
        }
        let stream_id: [u8; 4] = value[stream_id_start..stream_id_start + 4]
            .try_into()
            .unwrap();
        let stream_id = u31::from_bytes(stream_id);

        Ok(PushPromisePayload {
            PadLength,
            PromisedStreamId: stream_id,
            HeaderBlockFragment: <Hpack as From<Vec<u8>>>::from(
                value[header_start..header_end].to_vec(),
            ),
            Padding: Some(value[header_end..].to_vec()),
        })
    }
}

impl FromBytes<PingPayload> for PingPayload {
    fn from(value: Vec<u8>, flags: u8) -> Result<PingPayload, FromBytesError> {
        let opaq_data: [u8; 8] = value[0..8].try_into().unwrap();
        let opaq_data = u64::from_be_bytes(opaq_data);
        Ok(Self {
            OpaqueData: opaq_data,
        })
    }
}

impl FromBytes<GoAwayPayload> for GoAwayPayload {
    fn from(value: Vec<u8>, flag: u8) -> Result<Self, FromBytesError> {
        let stream_id: [u8; 4] = value[0..4].try_into().unwrap();
        let stream_id = u31::from_bytes(stream_id);

        let error_code: [u8; 4] = value[4..8].try_into().unwrap();
        let error_code = u32::from_be_bytes(error_code);

        Ok(GoAwayPayload {
            LastStreamId: stream_id,
            ErrorCode: error_code,
            AdditionalData: value[8..].to_vec(),
        })
    }
}

impl FromBytes<WindowUpdatePayload> for WindowUpdatePayload {
    fn from(value: Vec<u8>, flag: u8) -> Result<Self, FromBytesError> {
        let window_size: [u8; 4] = value[0..4].try_into().unwrap();
        let window_size = u32::from_be_bytes(window_size);

        Ok(WindowUpdatePayload {
            WindowSizeIncrement: window_size,
        })
    }
}

impl FromBytes<ContinuationPayload> for ContinuationPayload {
    fn from(value: Vec<u8>, flag: u8) -> Result<Self, FromBytesError> {
        Ok(ContinuationPayload {
            HeaderBlockFragment: <Hpack as From<Vec<u8>>>::from(value),
        })
    }
}

impl Payload {
    pub fn from(value: Vec<u8>, flag: u8, frame_type: FrameType) -> Result<Self, FromBytesError> {
        match frame_type {
            FrameType::Data => Ok(Payload::Data(
                <DataPayload as FromBytes<DataPayload>>::from(value, flag)?,
            )),

            FrameType::Headers => Ok(Payload::Headers(<HeadersPayload as FromBytes<
                HeadersPayload,
            >>::from(value, flag)?)),

            FrameType::Priority => Ok(Payload::Priority(<PriorityPayload as FromBytes<
                PriorityPayload,
            >>::from(value, flag)?)),

            FrameType::RstStream => Ok(Payload::RstStream(<RstStreamPayload as FromBytes<
                RstStreamPayload,
            >>::from(value, flag)?)),

            FrameType::Settings => Ok(Payload::Settings(<SettingsPayload as FromBytes<
                SettingsPayload,
            >>::from(value, flag)?)),

            FrameType::PushPromise => {
                Ok(Payload::PushPromise(<PushPromisePayload as FromBytes<
                    PushPromisePayload,
                >>::from(value, flag)?))
            }

            FrameType::Ping => Ok(Payload::Ping(
                <PingPayload as FromBytes<PingPayload>>::from(value, flag)?,
            )),

            FrameType::GoAway => Ok(Payload::GoAway(<GoAwayPayload as FromBytes<
                GoAwayPayload,
            >>::from(value, flag)?)),

            FrameType::WindowUpdate => {
                Ok(Payload::WindowUpdate(<WindowUpdatePayload as FromBytes<
                    WindowUpdatePayload,
                >>::from(value, flag)?))
            }

            FrameType::Continuation => {
                Ok(Payload::Continuation(<ContinuationPayload as FromBytes<
                    ContinuationPayload,
                >>::from(value, flag)?))
            }

            FrameType::Unknown => Err(FromBytesError::InvalidPayloadType),
        }
    }
}

impl len for DataPayload {
    fn binary_len(&self) -> usize {
        let mut result: usize = 0;
        if self.PadLength.is_some() {
            result += 1;
            result += self.PadLength.unwrap() as usize;
        }
        result += self.data.len();
        result
    }
}

impl len for PriorityPayload {
    fn binary_len(&self) -> usize {
        5
    }
}

impl len for HeadersPayload {
    fn binary_len(&self) -> usize {
        let mut result: usize = 0;
        if self.PadLength.is_some() {
            result += 1;
            result += self.PadLength.unwrap() as usize;
        }

        if self.Priority.is_some() {
            result += self.Priority.as_ref().unwrap().binary_len();
        }

        result += self.HeaderBlockFragment.binary_len();
        result
    }
}

impl len for RstStreamPayload {
    fn binary_len(&self) -> usize {
        4
    }
}

impl len for SettingsPayload {
    fn binary_len(&self) -> usize {
        self.settings.len() * 6
    }
}

impl len for PushPromisePayload {
    fn binary_len(&self) -> usize {
        let mut result: usize = 0;
        if self.PadLength.is_some() {
            result += 1;
            result += self.PadLength.unwrap() as usize;
        }

        result += 4; // PromisedStreamId
        result += self.HeaderBlockFragment.binary_len();
        result
    }
}

impl len for PingPayload {
    fn binary_len(&self) -> usize {
        8
    }
}

impl len for GoAwayPayload {
    fn binary_len(&self) -> usize {
        8 + self.AdditionalData.len()
    }
}

impl len for WindowUpdatePayload {
    fn binary_len(&self) -> usize {
        4
    }
}

impl len for ContinuationPayload {
    fn binary_len(&self) -> usize {
        self.HeaderBlockFragment.binary_len()
    }
}

impl len for Payload {
    fn binary_len(&self) -> usize {
        match self{
            Payload::Data(d) => d.binary_len(),
            Payload::Headers(d) => d.binary_len() ,
            Payload::Priority(d) => d.binary_len(),
            Payload::RstStream(d) => d.binary_len(),
            Payload::Settings(d) => d.binary_len(),
            Payload::PushPromise(d) => d.binary_len(),
            Payload::Ping(d) => d.binary_len(),
            Payload::GoAway(d) => d.binary_len(),
            Payload::WindowUpdate(d) => d.binary_len(),
            Payload::Continuation(d) => d.binary_len(),
        }
    }
}