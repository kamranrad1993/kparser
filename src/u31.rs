use std::{
    fmt, hash::Hash, ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub}
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct u31(u32);

impl u31 {
    const MAX: u32 = 0x7FFFFFFF;

    pub fn new(value: u32) -> Self {
        u31(value & Self::MAX)
    }

    pub fn to_u32(self) -> u32 {
        self.0
    }

    pub fn to_bytes(self) -> [u8; 4] {
        <Self as Into<Vec<u8>>>::into(self).try_into().unwrap()
    }

    pub fn from_bytes(value: [u8; 4]) -> Self {
        Self {
            0: u32::from_be_bytes(value) & 0x7FFFFFFF,
        }
    }
}

impl Add for u31 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        u31::new(self.0 + other.0)
    }
}

impl Sub for u31 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        u31::new(self.0 - other.0)
    }
}

impl Mul for u31 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        u31::new(self.0 * other.0)
    }
}

impl Div for u31 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        u31::new(self.0 / other.0)
    }
}

impl Rem for u31 {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        u31::new(self.0 % other.0)
    }
}

impl BitAnd for u31 {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        u31::new(self.0 & other.0)
    }
}

impl BitOr for u31 {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        u31::new(self.0 | other.0)
    }
}

impl BitXor for u31 {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        u31::new(self.0 ^ other.0)
    }
}

impl Not for u31 {
    type Output = Self;

    fn not(self) -> Self {
        u31::new(!self.0 & u31::MAX)
    }
}

impl Shl<u32> for u31 {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        u31::new(self.to_u32() << rhs)
    }
}

impl Shr<u32> for u31 {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        u31::new(self.to_u32() >> rhs)
    }
}

impl fmt::Display for u31 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_u32())
    }
}

impl From<u64> for u31 {
    fn from(value: u64) -> Self {
        u31::new((value & 0x7FFFFFFF) as u32)
    }
}

impl From<usize> for u31 {
    fn from(value: usize) -> Self {
        u31::new((value & 0x7FFFFFFF) as u32)
    }
}

impl From<u31> for u64 {
    fn from(value: u31) -> Self {
        value.to_u32() as u64
    }
}

impl From<i32> for u31{
    fn from(value: i32) -> Self {
        u31::new(value as u32)
    }
}

impl Into<Vec<u8>> for u31 {
    fn into(self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

impl Into<u32> for u31 {
    fn into(self) -> u32 {
        self.to_u32()
    }
}

// impl Hash for u31 {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.to_u32().hash(state);
//     }
// }