use std::ops::{BitAnd, BitOr};

// bitflags::bitflags! {
//     #[derive(Debug)]
//     pub struct DataPayloadFlag: u8 {
//         const PADDED = 0x08;
//     }

//     #[derive(Debug)]
//     pub struct HeadersPayloadFlag: u8 {
//         const END_STREAM = 0x01;
//         const END_HEADERS = 0x04;
//         const PRIORITY = 0x08;
//     }

//     #[derive(Debug)]
//     pub struct PriorityPayloadFlag: u8 {}

//     #[derive(Debug)]
//     pub struct RstStreamPayloadFlag: u8 {}

//     #[derive(Debug)]
//     pub struct SettingPayloadFlag: u8 {
//         const ACK = 0x01;
//     }

//     #[derive(Debug)]
//     pub struct PushPromisePayloadFlag: u8 {
//         const END_PUSH_PROMISE = 0x01;
//     }

//     #[derive(Debug)]
//     pub struct CoawayPayloadFlag: u8 {}

//     #[derive(Debug)]
//     pub struct WindowUpdatePayloadFlag: u8{}

//     #[derive(Debug)]
//     pub struct ContinuationPayloadFlag: u8 {
//         const END_HEADERS = 0x04;
//     }
// }

// #[derive(Debug)]
// pub enum DataPayloadFlag {
//     PADDED = 0x08,
// }

// #[derive(Debug)]
// pub enum HeadersPayloadFlag {
//     END_STREAM = 0x01,
//     END_HEADERS = 0x04,
//     PRIORITY = 0x08,
// }

// #[derive(Debug)]
// pub enum PriorityPayloadFlag {}

// #[derive(Debug)]
// pub enum RstStreamPayloadFlag {}

// #[derive(Debug)]
// pub enum SettingPayloadFlag {
//     ACK = 0x01,
// }

// #[derive(Debug)]
// pub enum PushPromisePayloadFlag {
//     END_PUSH_PROMISE = 0x01,
// }

// #[derive(Debug)]
// pub enum CoawayPayloadFlag {}

// #[derive(Debug)]
// pub enum WindowUpdatePayloadFlag {}

// #[derive(Debug)]
// pub enum ContinuationPayloadFlag {
//     END_HEADERS = 0x04,
// }

// #[derive(Debug)]
// pub enum Flag {
//     DATA(DataPayloadFlag),
//     HEADERS(HeadersPayloadFlag),
//     PRIORITY(PriorityPayloadFlag),
//     RST_STREAM(RstStreamPayloadFlag),
//     SETTING(SettingPayloadFlag),
//     PUSH_PROMISE(PushPromisePayloadFlag),
//     COAWAY(CoawayPayloadFlag),
//     wINDOW_UPDATE(WindowUpdatePayloadFlag),
//     CONTINUATION(ContinuationPayloadFlag),
// }

// impl BitAnd for DataPayloadFlag {
//     type Output = DataPayloadFlag;

//     fn bitand(self, rhs: Self) -> Self::Output {
//         let result = (self as u8) & (rhs as u8);
//         match result {
//             0x08 => DataPayloadFlag::PADDED,
//             _ => unreachable!(),
//         }
//     }
// }

// impl BitOr for DataPayloadFlag{
//     type Output=DataPayloadFlag;

//     fn bitor(self, rhs: Self) -> Self::Output {

//     }
// }

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

pub mod PingPayloadFlag{
    pub const ACK: u8 = 0x01;
}

pub mod CoawayPayloadFlag {}

pub mod WindowUpdatePayloadFlag {}

pub mod ContinuationPayloadFlag {
    pub const END_HEADERS: u8 = 0x04;
}

#[derive(Debug)]
pub enum Flag {
    DATA(u8),
    HEADERS(u8),
    PRIORITY(u8),
    RST_STREAM(u8),
    SETTING(u8),
    PUSH_PROMISE(u8),
    COAWAY(u8),
    wINDOW_UPDATE(u8),
    CONTINUATION(u8),
}
