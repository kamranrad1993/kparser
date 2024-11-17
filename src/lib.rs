pub mod u24;
pub mod u31;

pub mod http2;
pub use http2::Http2Pri;

pub mod http;

#[cfg(test)]
mod tests {
    use http2::{DataPayload, Payload};

    use super::*;

    #[test]
    fn it_works() {}
}


#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}