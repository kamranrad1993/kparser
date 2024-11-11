pub mod DataPayloadFlag {
    pub const PADDED: u8 = 0x08;
    pub const END_STREAM: u8 = 0x01;
}

pub mod HeadersPayloadFlag {
    pub const END_STREAM: u8 = 0x01;
    pub const END_HEADERS: u8 = 0x04;
    pub const PADDED: u8 = 0x08;
    pub const PRIORITY: u8 = 0x20;
}

pub mod PriorityPayloadFlag {}

pub mod RstStreamPayloadFlag {}

pub mod SettingPayloadFlag {
    pub const ACK: u8 = 0x01;
}

pub mod PushPromisePayloadFlag {
    pub const PADDED: u8 = 0x08;
    pub const END_HEADERS: u8 = 0x01;
}

pub mod PingPayloadFlag {
    pub const ACK: u8 = 0x01;
}

pub mod CoawayPayloadFlag {}

pub mod WindowUpdatePayloadFlag {}

pub mod ContinuationPayloadFlag {
    pub const END_HEADERS: u8 = 0x04;
}

// #[derive(Debug)]
// pub enum Flag {
//     DATA(u8),
//     HEADERS(u8),
//     PRIORITY(u8),
//     RST_STREAM(u8),
//     SETTING(u8),
//     PUSH_PROMISE(u8),
//     COAWAY(u8),
//     wINDOW_UPDATE(u8),
//     CONTINUATION(u8),
// }
