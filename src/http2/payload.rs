use super::{hpack::HpackHeaders, payload_flags::SettingPayloadFlag};

#[derive(Debug)]
pub struct DataPayload {
    PadLength: Option<u8>,
    data: Vec<u8>,
    Padding: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct HeadersPayload {
    PadLength: Option<u8>,
    Priority: Option<PriorityPayload>,
    HeaderBlockFragment: HpackHeaders,
    Padding: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct PriorityPayload {
    ExclusiveFlag: bool,
    StreamDependency: u32,
    Weight: u8,
}

#[derive(Debug)]
pub struct RstStreamPayload {
    ErrorCode: u32,
}

type SettingIdentifier = u16;
type SettingValue = u32;
#[derive(Debug)]
pub struct SettingsPayload {
    settings: Vec<(SettingIdentifier, SettingValue)>,
}

#[derive(Debug)]
pub struct PushPromisePayload {
    PadLength: Option<u8>,
    PromisedStreamId: u32,
    HeaderBlockFragment: HpackHeaders,
    Padding: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct GoAwayPayload {
    LastStreamId: u32, // maybe u31
    ErrorCode: u32,
    AdditionalData: Vec<u8>,
}

#[derive(Debug)]
pub struct WindowUpdatePayload {
    WindowSizeIncrement: u32, //31 bit intiger
}

#[derive(Debug)]
pub struct ConfigurationPayload {
    HeaderBlockFragment: HpackHeaders,
}

#[derive(Debug)]
pub enum Payload {
    Data(DataPayload),
    Headers(HeadersPayload),
    Priority(PriorityPayload),
    RstStream(RstStreamPayload),
    Settings(SettingsPayload),
    PushPromise(PushPromisePayload),
    GoAway(GoAwayPayload),
    WindowUpdate(WindowUpdatePayload),
    Configuration(ConfigurationPayload)
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
        result.push(value)
    }
}

impl Into<Vec<u8>> for HeadersPayload {
    fn into(self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut data = Vec::new();
        if let Some(pad_length) = self.PadLength {
            data.push(pad_length);
        }
        if let Some(priority) = self.Priority {
            // data.push(pad_length);
        }

        if let Some(padding) = self.Padding {
            data.extend(padding);
        }
        data
    }
}
