//! The [`Word`] trait, its super traits and its implementations for all signed integer types.

use core::fmt::{Debug, Display};
use core::num::ParseIntError;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign,
    Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

/// The `WordBase` trait defines the base trait constraints for the Word type.
/// It has a blanket implementation for all types that implement its super traits.
pub trait WordBase: Debug + Display + Copy + Eq + Ord + Default {}

impl<T> WordBase for T where T: Debug + Display + Copy + Eq + Ord + Default {}

/// The `WordConvert` trait defines the convertion trait constraints for the Word type.
/// It has a blanket implementation for all types that implement its super traits.
pub trait WordConvert: TryFrom<usize> + Into<usize> + From<i32> {}

impl<T> WordConvert for T where T: TryFrom<usize> + Into<usize> + From<i32> {}

/// The `WordOps` trait defines operation trait constraints for the Word type.
/// It has a blanket implementation for all types that implement its super traits.
pub trait WordOps:
    Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Neg
    + Rem
    + RemAssign
{
}

impl<T> WordOps for T where
    T: Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + Neg
{
}

/// The `WordBitOps` trait defines bitwise operation trait constraints for the Word type.
/// It has a blanket implementation for all types that implement its super traits.
pub trait WordBitOps:
    Sized
    + BitAnd<Self, Output = Self>
    + BitOr<Self, Output = Self>
    + BitXor<Self, Output = Self>
    + Not<Output = Self>
    + Shl<Self, Output = Self>
    + Shr<Self, Output = Self>
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
    + ShlAssign
    + ShrAssign
{
}

impl<T> WordBitOps for T where
    T: BitAnd<Self, Output = Self>
        + BitOr<Self, Output = Self>
        + BitXor<Self, Output = Self>
        + Not<Output = Self>
        + Shl<Self, Output = Self>
        + Shr<Self, Output = Self>
        + BitAndAssign
        + BitOrAssign
        + BitXorAssign
        + ShlAssign
        + ShrAssign
{
}

/// The Word trait wraps the underlying type used as the processor’s word size, mimicking real-world architectures
/// (e.g., [`I32`] corresponds to a 32-bit architecture).
///
/// The [`Word`] trait is implemented for the following signed integer types:
/// - [`I8`]
/// - [`I16`]
/// - [`I32`]
/// - [`I64`]
/// - [`I128`]
/// - [`ISize`]
/// 
/// These types use two's complement representation, mirroring how real-world processor architectures work.
/// To implement custom [`Word`] types, you can define your own type that implements the [`Word`] trait.
pub trait Word: WordBase + WordConvert + WordOps + WordBitOps {
    /// This is a wrapper around the [`from_str_radix()`](i32::from_str_radix()) function that is implemented for all of Rust's numeric types.
    ///
    /// # Errors
    /// Returns [`ParseIntError`] when the parsing failed.
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError>;

    /// Checks for carry when adding.
    #[must_use]
    fn check_carry_add(&self, rhs: Self) -> bool;

    /// Checks for carry when subtracting.
    #[must_use]
    fn check_carry_sub(&self, rhs: Self) -> bool;

    /// Checks for carry when multiplying.
    #[must_use]
    fn check_carry_mul(&self, rhs: Self) -> bool;

    /// Checks for division overflow (i.e., MIN / -1 for signed types).
    /// Similiar to [`Word::overflowing_div()`] this is a convenience wrapper over Rust's [`overflowing_div()`](i32::overflowing_div()).
    /// However it discards the result of the division.
    #[must_use]
    fn check_carry_div(&self, rhs: Self) -> bool;

    /// Convenience wrapper over Rust's [`overflowing_add()`](i32::overflowing_add()).
    #[must_use]
    fn overflowing_add(&self, rhs: Self) -> (Self, bool);
    /// Convenience wrapper over Rust's [`overflowing_sub()`](i32::overflowing_sub()).
    #[must_use]
    fn overflowing_sub(&self, rhs: Self) -> (Self, bool);
    /// Convenience wrapper over Rust's [`overflowing_mul()`](i32::overflowing_mul()).
    #[must_use]
    fn overflowing_mul(&self, rhs: Self) -> (Self, bool);
    /// Convenience wrapper over Rust's [`overflowing_div()`](i32::overflowing_div()).
    #[must_use]
    fn overflowing_div(&self, rhs: Self) -> (Self, bool);

    /// Convenience wrapper over Rust's [`rotate_left()`](i32::rotate_left()).
    #[must_use]
    fn rotate_left(&self, val: u32) -> Self;
    /// Convenience wrapper over Rust's [`rotate_right()`](i32::rotate_right()).
    #[must_use]
    fn rotate_right(&self, val: u32) -> Self;
}

// Implements the From<i32> trait for a wrapper struct around another type like i8.
// It is necessary as Word is implemented for all signed types also i32.
// From<i32> cannot be implemented for i32 and therefore this extra macro is needed.
macro_rules! from_i32 {
    ($name: ident, $type: ty $(,)? ) => {
        impl ::core::convert::From<i32> for $name {
            fn from(value: i32) -> Self {
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_lossless)]
                #[allow(clippy::cast_sign_loss)]
                $name(value as $type)
            }
        }
    };
}

// Implements the Word trait for a wrapper struct around another type like i8.
macro_rules! impl_word {
    ($name: ident, $type: ty $(,)? ) => {
        #[doc = concat!("Wrapper struct around ", stringify!($type), ".")]
        #[doc = concat!("Represents a ", stringify!($type), "-bit processor architecture.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[repr(transparent)]
        pub struct $name($type);

        impl Word for $name {
            fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
                <$type>::from_str_radix(s, radix).map($name)
            }

            fn check_carry_add(&self, rhs: Self) -> bool {
                #[allow(clippy::cast_sign_loss)]
                let (lhs, rhs) = (self.0 as u128, rhs.0 as u128);
                lhs + rhs > <$type>::MAX as u128
            }

            fn check_carry_sub(&self, rhs: Self) -> bool {
                #[allow(clippy::cast_sign_loss)]
                let (lhs, rhs) = (self.0 as u128, rhs.0 as u128);
                lhs < rhs
            }

            fn check_carry_mul(&self, rhs: Self) -> bool {
                #[allow(clippy::cast_sign_loss)]
                let (lhs, rhs) = (self.0 as u128, rhs.0 as u128);
                lhs * rhs > <$type>::MAX as u128
            }

            fn check_carry_div(&self, rhs: Self) -> bool {
                self.0.overflowing_div(rhs.0).1
            }

            fn overflowing_add(&self, rhs: Self) -> (Self, bool) {
                let (res, overflow) = self.0.overflowing_add(rhs.0);
                (Self(res), overflow)
            }

            fn overflowing_sub(&self, rhs: Self) -> (Self, bool) {
                let (res, overflow) = self.0.overflowing_sub(rhs.0);
                (Self(res), overflow)
            }

            fn overflowing_mul(&self, rhs: Self) -> (Self, bool) {
                let (res, overflow) = self.0.overflowing_mul(rhs.0);
                (Self(res), overflow)
            }

            fn overflowing_div(&self, rhs: Self) -> (Self, bool) {
                let (res, overflow) = self.0.overflowing_div(rhs.0);
                (Self(res), overflow)
            }

            fn rotate_left(&self, val: u32) -> Self {
                Self(self.0.rotate_left(val))
            }

            fn rotate_right(&self, val: u32) -> Self {
                Self(self.0.rotate_right(val))
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error> {
                write!(f, "{}", self.0)
            }
        }

        impl ::core::convert::From<$name> for usize {
            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_possible_truncation)]
            fn from(value: $name) -> usize {
                value.0 as usize
            }
        }

        impl ::core::convert::TryFrom<usize> for $name {
            type Error = ::core::num::TryFromIntError;

            fn try_from(value: usize) -> Result<Self, Self::Error> {
                Ok(Self(<$type>::try_from(value)?))
            }
        }

        impl ::core::convert::From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }

        impl ::core::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self(self.0.wrapping_add(rhs.0))
            }
        }

        impl ::core::ops::AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }

        impl ::core::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self(self.0.wrapping_sub(rhs.0))
            }
        }

        impl ::core::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other;
            }
        }

        impl ::core::ops::Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                Self(self.0.wrapping_mul(rhs.0))
            }
        }

        impl ::core::ops::MulAssign for $name {
            fn mul_assign(&mut self, other: Self) {
                *self = *self * other;
            }
        }

        impl ::core::ops::Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                Self(self.0.wrapping_div(rhs.0))
            }
        }

        impl ::core::ops::DivAssign for $name {
            fn div_assign(&mut self, other: Self) {
                *self = *self / other;
            }
        }

        impl ::core::ops::Neg for $name {
            type Output = Self;

            fn neg(self) -> Self {
                Self(self.0.wrapping_neg())
            }
        }

        impl ::core::ops::Rem for $name {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self {
                Self(self.0.wrapping_rem(rhs.0))
            }
        }

        impl ::core::ops::RemAssign for $name {
            fn rem_assign(&mut self, rhs: Self) {
                *self = *self % rhs
            }
        }

        impl ::core::ops::BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl ::core::ops::BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl ::core::ops::BitXor for $name {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }

        impl ::core::ops::Not for $name {
            type Output = Self;

            fn not(self) -> Self {
                Self(!self.0)
            }
        }

        impl ::core::ops::Shl for $name {
            type Output = Self;

            fn shl(self, rhs: Self) -> Self {
                Self(self.0 << rhs.0)
            }
        }

        impl ::core::ops::Shr for $name {
            type Output = Self;

            fn shr(self, rhs: Self) -> Self {
                Self(self.0 >> rhs.0)
            }
        }

        impl ::core::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl ::core::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl ::core::ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }

        impl ::core::ops::ShlAssign for $name {
            fn shl_assign(&mut self, rhs: Self) {
                self.0 <<= rhs.0;
            }
        }

        impl ::core::ops::ShrAssign for $name {
            fn shr_assign(&mut self, rhs: Self) {
                self.0 >>= rhs.0;
            }
        }
    };
}

impl_word!(I8, i8);
impl_word!(I16, i16);
impl_word!(I32, i32);
impl_word!(I64, i64);
impl_word!(I128, i128);
impl_word!(ISize, isize);

from_i32!(I8, i8);
from_i32!(I16, i16);
from_i32!(I64, i64);
from_i32!(I128, i128);
from_i32!(ISize, isize);
