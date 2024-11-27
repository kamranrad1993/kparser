#![feature(try_trait_v2_residual)]
#![allow(try_trait_v2_residual)]pub mod u24;
pub mod u31;

pub mod http2;

use std::ops;

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
impl<T, E> From<Result<T, E>> for std::result::Result<T, E> {
    fn from(val: Result<T, E>) -> Self {
        match val{
            Result::Ok(value) => std::result::Result::Ok(value),
            Result::Err(error) => std::result::Result::Err(error),
        }
    }
}
impl<T, E> From<std::result::Result<T, E>> for Result<T, E> {
    fn from(val: std::result::Result<T, E>) -> Self {
        match val{
            std::result::Result::Ok(value) => Result::Ok(value),
            std::result::Result::Err(error) => Result::Err(error),
        }
    }
}
#[unstable(feature = "try_trait_v2_residual", issue = "91285")]
impl<T, E> ops::Residual<T> for Result<T, E> {
    type TryType = Result<T, E>;
}
