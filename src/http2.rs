pub mod frame;
pub mod hpack;
pub mod payload;
pub mod payload_flags;
pub mod huffman;

use std::str::FromStr;

pub use frame::*;
pub use hpack::*;
pub use payload::*;
pub use payload_flags::*;
pub use huffman::*;

trait len {
    fn binary_len(&self) -> usize;
}

const DEFAULT_PRI: &str = "PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
pub struct Http2Pri {
    pub content: String,
}

#[derive(Debug)]
pub enum Http2PriErr {
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

impl Http2Pri {
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
    use crate::u24::u24;
    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream},
        vec,
    };

    use super::*;

    fn read_frame(buf: Vec<u8>) -> usize {
        let mut frame = <Frame as From<Vec<u8>>>::from(buf);
        let len = frame.binary_len();
        println!("frame type : {}", frame.frame_type);
        println!("frame length : {}", frame.length);
        println!("stream id : {}", frame.stream_id);
        match &mut frame.payload{
            Payload::Data(data) => {
                println!("receive len {}", data.data.len());
            //    let s = std::str::from_utf8(data.data.as_slice()).unwrap();
            //    println!("received data : {s}");
            },
            Payload::Headers(headers) => {
                println!("Headers: ");
                for i in headers.HeaderBlockFragment.decode().unwrap() {
                    let key = String::from_utf8_lossy(&i.0);
                    let value = String::from_utf8_lossy(&i.1);

                    println!("  {}: {}", key, value);
                }
            },
            Payload::Priority(_) => println!("payload:  Priority"),
            Payload::RstStream(_) => println!("payload:  RstStream"),
            Payload::Settings(_) => println!("payload:  Settings"),
            Payload::PushPromise(_) => println!("payload:  PushPromise"),
            Payload::Ping(_) => println!("payload:  Ping"),
            Payload::GoAway(_) => println!("payload:  GoAway"),
            Payload::WindowUpdate(_) => println!("payload:  WindowUpdate"),
            Payload::Continuation(_) => println!("payload:  Continuation"),
        }
        println!("+++++++++++++++++++++++++++++++++++++++++++++++++++");
        len
    }

    #[test]
    fn it_works() {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        let (tcp_stream, address) = &mut listener.accept().unwrap();
        println!("new connection");

        let mut buf = vec![0u8; 8192];
        let size = tcp_stream.read(&mut buf).unwrap();
        buf = buf[0..size].to_vec();

        let pri = Http2Pri::read_and_remove(&mut buf).unwrap();

        // let frame = <Frame as From<Vec<u8>>>::from(buf);
        // println!("frame type : {}" , frame.frame_type);
        // println!("frame length : {}" , frame.length);

        let mut buf_index = 0;
        loop {
            if buf_index >= buf.len() {
                break;
            }
            let frame_len = read_frame(buf[buf_index..].to_vec());
            buf_index += frame_len;
            // buf_index += 1;
            // println!("buf index {buf_index}")
        }

        let data_res = "hello".as_bytes().to_vec();
        let data_res_len: u24 = data_res.len().into();
        let payload_res = DataPayload{
            PadLength: None,
            data: data_res,
            Padding: None,
        };
        let frame_res = Frame{
            length: data_res_len,
            frame_type: FrameType::Data,
            flags: 0,
            stream_id: 2,
            payload: Payload::Data(payload_res),
        };
        let res: Vec<u8> = frame_res.into();
        tcp_stream.write(res.as_slice()).unwrap();
    }
}
