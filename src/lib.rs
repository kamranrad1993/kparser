
mod int31;

pub mod http2;
pub use http2::Http2Pri;



#[cfg(test)]
mod tests {
    use http2::{DataPayload, Payload};

    use super::*;

    #[test]
    fn it_works() {

        let  l = DataPayload{
            PadLength: None,
            data: vec![0u8;8],
            Padding: None,
        };
        let p = Payload::Data(l);
    }
}
