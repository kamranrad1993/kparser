use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Not};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Int31(u32);

impl Int31 {
    const MAX: u32 = 0x7FFFFFFF; // 31-bit max value: 2^31 - 1

    // Constructor that ensures the value fits within 31 bits
    pub fn new(value: u32) -> Self {
        Int31(value & Self::MAX) // Ensure the value is within 31 bits
    }

    // Get the inner value
    pub fn value(self) -> u32 {
        self.0
    }
}

impl Add for Int31 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Int31::new(self.0 + other.0)
    }
}

impl Sub for Int31 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Int31::new(self.0 - other.0)
    }
}

impl Mul for Int31 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Int31::new(self.0 * other.0)
    }
}

impl Div for Int31 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Int31::new(self.0 / other.0)
    }
}

impl Rem for Int31 {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Int31::new(self.0 % other.0)
    }
}

impl BitAnd for Int31 {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Int31::new(self.0 & other.0)
    }
}

impl BitOr for Int31 {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Int31::new(self.0 | other.0)
    }
}

impl BitXor for Int31 {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Int31::new(self.0 ^ other.0)
    }
}

impl Not for Int31 {
    type Output = Self;

    fn not(self) -> Self {
        Int31::new(!self.0 & Int31::MAX)
    }
}

impl Into<Vec<u8>> for Int31 {
    fn into(self) -> Vec<u8> {
            
    }
}