use core::fmt::{Debug, Display};
use core::num::ParseIntError;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

/// Marker trait for types that can be used as words in a stack.
pub trait Word:
    Debug
    + Display
    + Copy
    + Default
    + Into<usize>
    + From<usize>
    + From<i32>
    + Eq
    + Ord
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Neg<Output = Self>
    + Rem<Self, Output = Self>
    + RemAssign
{
    /// This is a wrapper around the `from_str_radix` function that is implemented for all of Rust's numeric types.
    ///
    /// # Errors
    /// Returns `ParseIntError` when the parsing failed.
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError>;

    fn carry_add(&self, rhs: Self) -> bool;
    fn carry_sub(&self, rhs: Self) -> bool;
    fn carry_mul(&self, rhs: Self) -> bool;
    fn carry_div(&self, rhs: Self) -> bool;

    fn overflowing_add(&self, rhs: Self) -> (Self, bool);
    fn overflowing_sub(&self, rhs: Self) -> (Self, bool);
    fn overflowing_mul(&self, rhs: Self) -> (Self, bool);
    fn overflowing_div(&self, rhs: Self) -> (Self, bool);
}

/// This macro is used to implement the From<i32> trait.
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

/// This macro can be used to implement the Word trait for a Wrapper struct around another type like i8.
macro_rules! impl_word {
    ($name: ident, $type: ty $(,)? ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[repr(transparent)]
        pub struct $name($type);

        impl Word for $name {
            /// This is a wrapper around the `from_str_radix` function that is implemented for all of Rust's signed numeric types.
            ///
            /// # Errors
            /// Returns `ParseIntError` when the parsing failed.
            fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
                <$type>::from_str_radix(s, radix).map($name)
            }

            /// Checks for carry when adding.
            fn carry_add(&self, rhs: Self) -> bool {
                #[allow(clippy::cast_sign_loss)]
                let (lhs, rhs) = (self.0 as u128, rhs.0 as u128);
                lhs + rhs > <$type>::MAX as u128
            }

            /// Checks for carry when subtracting.
            fn carry_sub(&self, rhs: Self) -> bool {
                #[allow(clippy::cast_sign_loss)]
                let (lhs, rhs) = (self.0 as u128, rhs.0 as u128);
                lhs < rhs
            }

            /// Checks for carry when multiplying.
            fn carry_mul(&self, rhs: Self) -> bool {
                #[allow(clippy::cast_sign_loss)]
                let (lhs, rhs) = (self.0 as u128, rhs.0 as u128);
                lhs * rhs > <$type>::MAX as u128
            }

            /// Checks for division overflow (i.e., MIN / -1 for signed types).
            /// Similiar to [`Word::overflowing_div()`] this is a convenience wrapper over `overflowing_div()`.
            /// However it discards the result of the division.
            fn carry_div(&self, rhs: Self) -> bool {
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

        impl ::core::convert::From<usize> for $name {
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_possible_wrap)]
            fn from(value: usize) -> Self {
                Self(value as $type)
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
