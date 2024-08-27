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

#[derive(Debug)]
pub enum Http2PriErr{
    BufferSizeError,
    ParseError,
}

impl From<Vec<u8>> for Http2Pri {
    fn from(value: Vec<u8>) -> Self {
        Self {
            content: String::from_str(std::str::from_utf8(&value[0..24]).unwrap()).unwrap(),
        }
    }
}

impl Http2Pri{
    pub fn read_and_remove(buffer: &mut Vec<u8>) -> Result<Http2Pri, Http2PriErr> {
        if buffer.len() < 24 {
            return Err(Http2PriErr::BufferSizeError);
        }

        let result = <Http2Pri as From<Vec<u8>>>::from(buffer.clone());
        buffer.drain(0..24);
        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use std::{io::{Read, Write}, net::{
        TcpListener,
        TcpStream
    }, vec};
    use super::*;

    #[test]
    fn it_works() {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        let (tcp_stream, address) = &mut listener.accept().unwrap();
        println!("new connection");

        let mut buf = vec![0u8;8192];
        let size = tcp_stream.read(&mut buf).unwrap();

        let pri = Http2Pri::read_and_remove(&mut buf).unwrap();
        
        let frame = <Frame as From<Vec<u8>>>::from(buf);
        println!("frame type : {}" , frame.frame_type);

        match frame.payload{
            Payload::Data(data) => {
                println!("receive len {}", data.data.len());
            //    let s = std::str::from_utf8(data.data.as_slice()).unwrap();
            //    println!("received data : {s}");
            },
            _ => {}
            // Payload::Headers(_) => todo!(),
            // Payload::Priority(_) => todo!(),
            // Payload::RstStream(_) => todo!(),
            // Payload::Settings(_) => todo!(),
            // Payload::PushPromise(_) => todo!(),
            // Payload::Ping(_) => todo!(),
            // Payload::GoAway(_) => todo!(),
            // Payload::WindowUpdate(_) => todo!(),
            // Payload::Continuation(_) => todo!(),
        }

        let data_res = "hello".as_bytes().to_vec();
        let data_res_len = data_res.len() as u32;
        let payload_res = DataPayload{
            PadLength: None,
            data: data_res,
            Padding: None,
        };
        let frame_res = Frame{
            length: data_res_len,
            frame_type: FrameType::Data,
            flags: 0,
            stream_id: frame.stream_id,
            payload: Payload::Data(payload_res),
        };
        let res: Vec<u8> = frame_res.into();
        tcp_stream.write(res.as_slice()).unwrap();
    }
}
