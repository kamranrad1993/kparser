use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct u24([u8; 3]);

impl u24 {
    pub fn new(value: u32) -> Self {
        let bytes = [
            ((value >> 16) & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        ];
        u24(bytes)
    }

    pub fn to_u32(self) -> u32 {
        (self.0[0] as u32) << 16 | ((self.0[1] as u32) << 8) | ((self.0[2] as u32))
    }

    pub fn to_bytes(self) -> [u8; 3] {
        self.0
    }

    pub fn from_bytes(value: [u8; 3]) -> Self {
        Self { 0: value }
    }
}

impl Add for u24 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        u24::new(self.to_u32() + other.to_u32())
    }
}

impl Sub for u24 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        u24::new(self.to_u32() - other.to_u32())
    }
}

impl Mul for u24 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        u24::new(self.to_u32() * other.to_u32())
    }
}

impl Div for u24 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        u24::new(self.to_u32() / other.to_u32())
    }
}

impl Rem for u24 {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        u24::new(self.to_u32() % other.to_u32())
    }
}

impl BitAnd for u24 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        u24::new(self.to_u32() & rhs.to_u32())
    }
}

impl BitOr for u24 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        u24::new(self.to_u32() | rhs.to_u32())
    }
}

impl BitXor for u24 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        u24::new(self.to_u32() ^ rhs.to_u32())
    }
}

impl Not for u24 {
    type Output = Self;

    fn not(self) -> Self::Output {
        u24::new(!self.to_u32() & 0xFFFFFF)
    }
}

impl Shl<u32> for u24 {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        u24::new(self.to_u32() << rhs)
    }
}

impl Shr<u32> for u24 {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        u24::new(self.to_u32() >> rhs)
    }
}

impl fmt::Display for u24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_u32())
    }
}


impl From<u64> for u24 {
    fn from(value: u64) -> Self {
        u24::new((value & 0xFFFFFF) as u32)
    }
}

impl From<usize> for u24 {
    fn from(value: usize) -> Self {
        u24::new((value & 0xFFFFFF) as u32)
    }
}

impl From<u24> for u64 {
    fn from(value: u24) -> Self {
        value.to_u32() as u64
    }
}
