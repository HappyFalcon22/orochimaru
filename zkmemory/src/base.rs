use core::fmt::{Debug, Display};
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::usize;
use ethnum::U256;

/// Base trait for memory address and value
pub trait Base<const S: usize = 0, T = Self>:
    Ord
    + Copy
    + PartialEq
    + Eq
    + Ord
    + PartialOrd
    + Display
    + Debug
    + From<i32>
    + Into<i32>
    + From<usize>
    + Into<usize>
    + From<u64>
    + Into<u64>
    + From<[u8; S]>
    + Into<[u8; S]>
    + Add<T, Output = T>
    + Mul<T, Output = T>
    + Sub<T, Output = T>
    + Rem<T, Output = T>
    + Div<T, Output = T>
{
    /// The max value of the cell
    const MAX: Self;
    /// The min value of the cell
    const MIN: Self;
    /// Cell size in Base
    const WORD_SIZE: Self;
    /// The size of the cell
    const WORD_USIZE: usize = S;
    /// Check if the value is zero
    fn is_zero(&self) -> bool;
    /// Get the zero value
    fn zero() -> Self;
}

/// Convert from/to [usize](core::usize)
pub trait UIntConvertible {
    /// Convert from [usize](core::usize)
    fn from_usize(value: usize) -> Self;
    /// Convert to [usize](core::usize)
    fn to_usize(&self) -> usize;
}

/// Uint256 is a wrapper of [U256](ethnum::U256) to implement [Base](crate::base::Base)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Uint<T>(T);

impl<T: Display> Display for Uint<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Div<Output = T>> Div for Uint<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl<T: Add<Output = T>> Add for Uint<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Sub<Output = T>> Sub for Uint<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T: Rem<Output = T>> Rem for Uint<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl<T: Mul<Output = T>> Mul for Uint<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

macro_rules! new_base {
    (U256, $byte_size: expr) => {
        impl Base<32> for Uint<U256> {
            const MAX: Self = Self(U256::MAX);

            const MIN: Self = Self(U256::MIN);

            const WORD_SIZE: Self = Self(U256::new(32));

            fn is_zero(&self) -> bool {
                self.0 == U256::ZERO
            }

            fn zero() -> Self {
                Self(U256::ZERO)
            }
        }

        impl From<i32> for Uint<U256> {
            fn from(value: i32) -> Self {
                Self(U256::new(value as u128))
            }
        }

        impl From<usize> for Uint<U256> {
            fn from(value: usize) -> Self {
                Self(U256::new(value as u128))
            }
        }

        impl From<u64> for Uint<U256> {
            fn from(value: u64) -> Self {
                Self(U256::new(value as u128))
            }
        }

        impl Into<i32> for Uint<U256> {
            fn into(self) -> i32 {
                self.0.as_i32()
            }
        }

        impl Into<usize> for Uint<U256> {
            fn into(self) -> usize {
                self.0.as_usize()
            }
        }

        impl Into<u64> for Uint<U256> {
            fn into(self) -> u64 {
                self.0.as_u64()
            }
        }

        impl Into<[u8; 32]> for Uint<U256> {
            fn into(self) -> [u8; 32] {
                self.0.to_be_bytes()
            }
        }

        impl From<[u8; 32]> for Uint<U256> {
            fn from(value: [u8; 32]) -> Self {
                Self(U256::from_be_bytes(value))
            }
        }
    };
    ($primitive:ident, $byte_size: expr) => {
        impl Base<$byte_size> for Uint<$primitive> {
            const MAX: Self = Self($primitive::MAX);

            const MIN: Self = Self($primitive::MIN);

            const WORD_SIZE: Self = Self($byte_size as $primitive);

            fn is_zero(&self) -> bool {
                self.0 == 0
            }

            fn zero() -> Self {
                Self(0)
            }
        }

        impl From<i32> for Uint<$primitive> {
            fn from(value: i32) -> Self {
                Self(value as $primitive)
            }
        }

        impl From<usize> for Uint<$primitive> {
            fn from(value: usize) -> Self {
                Self(value as $primitive)
            }
        }

        impl From<u64> for Uint<$primitive> {
            fn from(value: u64) -> Self {
                Self(value as $primitive)
            }
        }

        impl Into<i32> for Uint<$primitive> {
            fn into(self) -> i32 {
                self.0 as i32
            }
        }

        impl Into<usize> for Uint<$primitive> {
            fn into(self) -> usize {
                self.0 as usize
            }
        }

        impl Into<u64> for Uint<$primitive> {
            fn into(self) -> u64 {
                self.0 as u64
            }
        }

        impl Into<[u8; $byte_size]> for Uint<$primitive> {
            fn into(self) -> [u8; $byte_size] {
                self.0.to_be_bytes()
            }
        }

        impl From<[u8; $byte_size]> for Uint<$primitive> {
            fn from(value: [u8; $byte_size]) -> Self {
                Self($primitive::from_be_bytes(value))
            }
        }
    };
}

new_base!(U256, 32);
new_base!(u64, 8);
new_base!(u32, 4);
new_base!(u16, 2);
