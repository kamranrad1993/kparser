#![feature(try_trait_v2_residual, try_trait_v2)]
#![allow(try_trait_v2_residual)]
pub mod u24;
pub mod u31;

pub mod http2;

use std::{
    convert,
    ops::{self, ControlFlow, FromResidual, Residual, Try},
};

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
pub enum Result<T, E>
where
    E: std::fmt::Debug,
    E: std::fmt::Display
{
    Ok(T),
    Err(E),
}
impl<T, E> From<Result<T, E>> for std::result::Result<T, E>
where
    E: std::fmt::Debug,
    E: std::fmt::Display
{
    fn from(val: Result<T, E>) -> Self {
        match val {
            Result::Ok(value) => std::result::Result::Ok(value),
            Result::Err(error) => std::result::Result::Err(error),
        }
    }
}
impl<T, E> From<std::result::Result<T, E>> for Result<T, E>
where
    E: std::fmt::Debug,
    E: std::fmt::Display
{
    fn from(val: std::result::Result<T, E>) -> Self {
        match val {
            std::result::Result::Ok(value) => Result::Ok(value),
            std::result::Result::Err(error) => Result::Err(error),
        }
    }
}

impl<T, E> std::ops::Try for Result<T, E>
where
    E: std::fmt::Debug,
    E: std::fmt::Display
{
    type Output = T;
    type Residual = Result<core::convert::Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        Self::Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Ok(value) => ControlFlow::Continue(value),
            Self::Err(err) => ControlFlow::Break(Result::Err(err)),
        }
    }
}
impl<T, E, F> std::ops::FromResidual<Result<core::convert::Infallible, F>> for Result<T, E>
where
    F: std::fmt::Debug,
    F: std::fmt::Display,
    E: From<F>,
    E: std::fmt::Debug,
    E: std::fmt::Display
{
    fn from_residual(residual: Result<core::convert::Infallible, F>) -> Self {
        match residual {
            Result::Err(err) => Result::Err(err.into()),
            _ => unreachable!(),
        }
    }
}

impl<T, E> Result<T, E>
where
    E: std::fmt::Debug,
    E: std::fmt::Display
{
    #[inline(always)]
    #[track_caller]
    pub fn unwrap(self) -> T
    where
        E: std::fmt::Debug,
    {
        match self {
            Result::Ok(t) => t,
            Result::Err(e) => panic!("called `Result::unwrap()` on an `Err` value {}", e),
        }
    }
}
