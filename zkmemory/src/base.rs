use core::fmt::{Debug, Display};
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::usize;
use ethnum::U256;

/// Base trait for memory address and value
pub trait Base<const S: usize, T = Self>:
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
    /// To big endian bytes
    fn fixed_be_bytes(&self) -> [u8; 32];
    /// To little endian bytes
    fn fixed_le_bytes(&self) -> [u8; 32];
}

/// Convert from/to [`core::usize`]
pub trait UIntConvertible {
    /// Convert from [`core::usize`]
    fn from_usize(value: usize) -> Self;
    /// Convert to [`core::usize`]
    fn to_usize(&self) -> usize;
}

/// Uint256 is a wrapper of [U256] to implement [Base]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Uint<T>(pub(crate) T);

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
        impl Base<$byte_size> for Uint<U256> {
            const MAX: Self = Self(U256::MAX);

            const MIN: Self = Self(U256::MIN);

            const WORD_SIZE: Self = Self(U256::new($byte_size));

            fn is_zero(&self) -> bool {
                self.0 == U256::ZERO
            }

            fn zero() -> Self {
                Self(U256::ZERO)
            }

            fn fixed_be_bytes(&self) -> [u8; 32] {
                self.0.to_be_bytes()
            }

            fn fixed_le_bytes(&self) -> [u8; 32] {
                self.0.to_le_bytes()
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

        impl From<Uint<U256>> for i32 {
            fn from(value: Uint<U256>) -> Self {
                value.0.as_i32()
            }
        }

        impl From<Uint<U256>> for usize {
            fn from(value: Uint<U256>) -> Self {
                value.0.as_usize()
            }
        }

        impl From<Uint<U256>> for u64 {
            fn from(value: Uint<U256>) -> Self {
                value.0.as_u64()
            }
        }

        impl From<Uint<U256>> for [u8; $byte_size] {
            fn from(value: Uint<U256>) -> Self {
                value.0.to_be_bytes()
            }
        }

        impl From<[u8; $byte_size]> for Uint<U256> {
            fn from(value: [u8; $byte_size]) -> Self {
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

            fn fixed_be_bytes(&self) -> [u8; 32] {
                let mut buf = [0u8; 32];
                buf[32 - $byte_size..].copy_from_slice(&self.0.to_be_bytes());
                buf
            }

            fn fixed_le_bytes(&self) -> [u8; 32] {
                let mut buf = [0u8; 32];
                buf[..$byte_size].copy_from_slice(&self.0.to_le_bytes());
                buf
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

        impl From<Uint<$primitive>> for i32 {
            fn from(value: Uint<$primitive>) -> Self {
                value.0 as i32
            }
        }

        impl From<Uint<$primitive>> for usize {
            fn from(value: Uint<$primitive>) -> Self {
                value.0 as usize
            }
        }

        impl From<Uint<$primitive>> for u64 {
            fn from(value: Uint<$primitive>) -> Self {
                value.0 as u64
            }
        }

        impl From<Uint<$primitive>> for [u8; $byte_size] {
            fn from(value: Uint<$primitive>) -> Self {
                value.0.to_be_bytes()
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
new_base!(u128, 16);
new_base!(u64, 8);
new_base!(u32, 4);
new_base!(u16, 2);

/// Uint256 is a wrapper of [U256] to implement [Base]
pub type B256 = Uint<U256>;
/// Uint128 is a wrapper of [u128](core::u128) to implement [Base]
pub type B128 = Uint<u128>;
/// Uint64 is a wrapper of [u64](core::u64) to implement [Base]
pub type B64 = Uint<u64>;
/// Uint32 is a wrapper of [u32](core::u32) to implement [Base]
pub type B32 = Uint<u32>;
/// Uint16 is a wrapper of [u16](core::u16) to implement [Base]
pub type B16 = Uint<u16>;

#[cfg(test)]
mod tests {
    use crate::base::{Base, B128, B256, B32, B64};

    #[test]
    fn base_struct_test() {
        // u256 test
        let chunk_zero = B256::zero();
        let bytes1 = [9u8; 32];
        let chunk1 = B256::from(bytes1);
        let bytes_convert: [u8; 32] = chunk1
            .try_into()
            .expect("Cannot convert from B256 to bytes");
        assert_eq!(bytes_convert, bytes1);
        assert!(chunk_zero.is_zero());
        assert!(!chunk1.is_zero());

        // u128 test
        let chunk_zero = B128::zero();
        let bytes1 = [9u8; 16];
        let chunk1 = B128::from(bytes1);
        let bytes_convert: [u8; 16] = chunk1
            .try_into()
            .expect("Cannot convert from B128 to bytes");
        assert_eq!(bytes_convert, bytes1);
        assert!(chunk_zero.is_zero());
        assert!(!chunk1.is_zero());

        // u64 test
        let chunk_zero = B64::zero();
        let bytes1 = [1u8; 8];
        let chunk1 = B64::from(bytes1);
        let bytes_convert: [u8; 8] = chunk1.try_into().expect("Cannot convert from B64 to bytes");
        assert_eq!(bytes_convert, bytes1);
        assert!(chunk_zero.is_zero());
        assert!(!chunk1.is_zero());

        // u32 test
        let chunk_zero = B64::zero();
        let bytes1 = [59u8; 8];
        let chunk1 = B64::from(bytes1);
        let bytes_convert: [u8; 8] = chunk1.try_into().expect("Cannot convert from B64 to bytes");
        assert_eq!(bytes_convert, bytes1);
        assert!(chunk_zero.is_zero());
        assert!(!chunk1.is_zero());
    }

    #[test]
    fn base_arithmetic_test() {
        // u256 test
        let chunk_1 = B256::from([34u8; 32]);
        let chunk_2 = B256::from([17u8; 32]);
        let chunk_3 = B256::from(5);
        let chunk_4 = B256::from(156);
        assert_eq!(chunk_1 + chunk_2, B256::from([51u8; 32]));
        assert_eq!(chunk_1 - chunk_2, B256::from([17u8; 32]));
        assert_eq!(chunk_4 * chunk_3, B256::from(156 * 5));
        assert_eq!(chunk_4 / chunk_3, B256::from(156 / 5));
        assert_eq!(chunk_4 % chunk_3, B256::from(156 % 5));

        // u128 test
        let chunk_1 = B128::from([19u8; 16]);
        let chunk_2 = B128::from([5u8; 16]);
        let chunk_3 = B128::from(7i32);
        let chunk_4 = B128::from(34u64);
        assert_eq!(chunk_1 + chunk_2, B128::from([24u8; 16]));
        assert_eq!(chunk_1 - chunk_2, B128::from([14u8; 16]));
        assert_eq!(chunk_4 * chunk_3, B128::from(34 * 7));
        assert_eq!(chunk_4 / chunk_3, B128::from(34 / 7));
        assert_eq!(chunk_4 % chunk_3, B128::from(34 % 7));

        // u64 test
        let chunk_1 = B64::from([61u8; 8]);
        let chunk_2 = B64::from([16u8; 8]);
        let chunk_3 = B64::from(12);
        let chunk_4 = B64::from(99);
        assert_eq!(chunk_1 + chunk_2, B64::from([77u8; 8]));
        assert_eq!(chunk_1 - chunk_2, B64::from([45u8; 8]));
        assert_eq!(chunk_4 * chunk_3, B64::from(99 * 12));
        assert_eq!(chunk_4 / chunk_3, B64::from(99 / 12));
        assert_eq!(chunk_4 % chunk_3, B64::from(99 % 12));

        // u32 test
        let chunk_1 = B32::from([34u8; 4]);
        let chunk_2 = B32::from([17u8; 4]);
        let chunk_3 = B32::from(5);
        let chunk_4 = B32::from(156);
        assert_eq!(chunk_1 + chunk_2, B32::from([51u8; 4]));
        assert_eq!(chunk_1 - chunk_2, B32::from([17u8; 4]));
        assert_eq!(chunk_4 * chunk_3, B32::from(156 * 5));
        assert_eq!(chunk_4 / chunk_3, B32::from(156 / 5));
        assert_eq!(chunk_4 % chunk_3, B32::from(156 % 5));
    }

    #[test]
    fn base_conversion_test() {
        // Test From<u256> traits
        let left = 5;
        let chunk1 = B256::from(5_usize);
        let right1 = i32::from(chunk1);
        let right2 = usize::from(chunk1);
        let right3 = u64::from(chunk1);
        assert_eq!(left, right1 as u64);
        assert_eq!(left, right2 as u64);
        assert_eq!(left, right3);

        // Test From<u256> traits
        let left = 5;
        let chunk1 = B128::from(5_usize);
        let right1 = i32::from(chunk1);
        let right2 = usize::from(chunk1);
        let right3 = u64::from(chunk1);
        assert_eq!(left, right1 as u64);
        assert_eq!(left, right2 as u64);
        assert_eq!(left, right3);

        // Test endianess of B256
        let num = B256::from(5);
        let chunk_be = {
            let mut buffer = [0u8; 32];
            buffer[31] = 5u8;
            buffer
        };
        let chunk_le = {
            let mut buffer = [0u8; 32];
            buffer[0] = 5u8;
            buffer
        };
        assert_eq!(num.fixed_be_bytes(), chunk_be);
        assert_eq!(num.fixed_le_bytes(), chunk_le);

        // Test endianess of B32
        let num = B32::from(10);
        let chunk_be = {
            let mut buffer = [0u8; 32];
            buffer[31] = 10u8;
            buffer
        };
        let chunk_le = {
            let mut buffer = [0u8; 32];
            buffer[0] = 10u8;
            buffer
        };
        assert_eq!(num.fixed_be_bytes(), chunk_be);
        assert_eq!(num.fixed_le_bytes(), chunk_le);
    }
}
