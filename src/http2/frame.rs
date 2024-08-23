use crate::http2::payload;
use crate::http2::payload_flags::Flag;

use super::payload::Payload;

#[derive(Debug)]
enum FrameType {
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
struct Frame {
    length: u32,
    frame_type: FrameType,
    flags: Flag,
    stream_id: u32,
    payload: Payload,
}

