use std::ops::{BitAnd, BitOr};

#[derive(Debug)]
pub enum DataPayloadFlag {
    PADDED = 0x08,
}

#[derive(Debug)]
pub enum HeadersPayloadFlag {
    END_STREAM = 0x01,
    END_HEADERS = 0x04,
    PRIORITY = 0x08,
}

#[derive(Debug)]
pub enum PriorityPayloadFlag {}

#[derive(Debug)]
pub enum RstStreamPayloadFlag {}

#[derive(Debug)]
pub enum SettingPayloadFlag {
    ACK = 0x01,
}

#[derive(Debug)]
pub enum PushPromisePayloadFlag {
    END_PUSH_PROMISE = 0x01,
}

#[derive(Debug)]
pub enum CoawayPayloadFlag {}

#[derive(Debug)]
pub enum WindowUpdatePayloadFlag {}

#[derive(Debug)]
pub enum ContinuationPayloadFlag {
    END_HEADERS = 0x04,
}

#[derive(Debug)]
pub enum Flag {
    DATA(DataPayloadFlag),
    HEADERS(HeadersPayloadFlag),
    PRIORITY(PriorityPayloadFlag),
    RST_STREAM(RstStreamPayloadFlag),
    SETTING(SettingPayloadFlag),
    PUSH_PROMISE(PushPromisePayloadFlag),
    COAWAY(CoawayPayloadFlag),
    wINDOW_UPDATE(WindowUpdatePayloadFlag),
    CONTINUATION(ContinuationPayloadFlag),
}

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
