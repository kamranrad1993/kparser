pub mod frame;
pub mod hpack;
pub mod payload;
pub mod payload_flags;

use std::str::FromStr;

pub use frame::*;
pub use hpack::*;
pub use payload::*;
pub use payload_flags::*;

const DEFAULT_PRI: &str = "PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
pub struct Http2Pri {
    pub content: String,
}

impl From<Vec<u8>> for Http2Pri {
    fn from(value: Vec<u8>) -> Self {
        Self {
            content: String::from_str(std::str::from_utf8(&value[0..24]).unwrap()).unwrap(),
        }
    }
}
