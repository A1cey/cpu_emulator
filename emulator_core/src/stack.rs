use core::fmt::{Debug, Display};
use core::num::ParseIntError;
use core::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use thiserror::Error;

use crate::instruction_set::InstructionSet;

/// Marker trait for types that can be used as words in a stack.
/// For negtive numbers a signed type must be used, e.g. i32.
pub trait Word:
    Debug
    + Display
    + Copy
    + Default
    + Into<usize>
    + From<i32>
    + Eq
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
    /// This is a wrapper around the `from_str_radix` function that is implemented for all of Rust's numeric types.
    ///
    /// # Errors
    /// Returns `ParseIntError` when the parsing failed.
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError>;
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

/// This macro can be used to implement the Word trait for a Wrapper struct around another type like u8.
macro_rules! impl_word {
    ($name: ident, $type: ty $(,)? ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[repr(transparent)]
        pub struct $name($type);

        impl Word for $name {
            /// This is a wrapper around the `from_str_radix` function that is implemented for all of Rust's numeric types.
            ///
            /// # Errors
            /// Returns `ParseIntError` when the parsing failed.
            fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
                <$type>::from_str_radix(s, radix).map($name)
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error> {
                write!(f, "{}", self.0)
            }
        }

        #[allow(clippy::from_over_into)]
        impl ::core::convert::Into<usize> for $name {
            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_possible_truncation)]
            fn into(self) -> usize {
                self.0 as usize
            }
        }

        impl ::core::convert::From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }

        impl ::core::ops::AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                *self = Self(self.0.wrapping_add(other.0));
            }
        }

        impl ::core::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                *self = Self(self.0.wrapping_sub(other.0));
            }
        }

        impl ::core::ops::MulAssign for $name {
            fn mul_assign(&mut self, other: Self) {
                *self = Self(self.0.wrapping_mul(other.0));
            }
        }

        impl ::core::ops::DivAssign for $name {
            fn div_assign(&mut self, other: Self) {
                *self = Self(self.0.wrapping_div(other.0));
            }
        }

        impl ::core::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self(self.0.wrapping_add(rhs.0))
            }
        }

        impl ::core::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self(self.0.wrapping_sub(rhs.0))
            }
        }

        impl ::core::ops::Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                Self(self.0.wrapping_mul(rhs.0))
            }
        }

        impl ::core::ops::Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                Self(self.0.wrapping_div(rhs.0))
            }
        }
    };
}

impl_word!(U8, u8);
impl_word!(U16, u16);
impl_word!(U32, u32);
impl_word!(U64, u64);
impl_word!(U128, u128);
impl_word!(USize, usize);
impl_word!(I8, i8);
impl_word!(I16, i16);
impl_word!(I32, i32);
impl_word!(I64, i64);
impl_word!(I128, i128);
impl_word!(ISize, isize);

// Implements From<i32> for all types except i32 as it already is defined for i32 using impl_word!
from_i32!(U8, u8);
from_i32!(U16, u16);
from_i32!(U32, u32);
from_i32!(U64, u64);
from_i32!(U128, u128);
from_i32!(USize, usize);
from_i32!(I8, i8);
from_i32!(I16, i16);
from_i32!(I64, i64);
from_i32!(I128, i128);
from_i32!(ISize, isize);

/// Stack
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Stack<const STACK_SIZE: usize, IS: InstructionSet<STACK_SIZE>>(pub [IS::W; STACK_SIZE]);

impl<const STACK_SIZE: usize, IS: InstructionSet<STACK_SIZE>> Deref for Stack<STACK_SIZE, IS> {
    type Target = [IS::W; STACK_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const STACK_SIZE: usize, IS: InstructionSet<STACK_SIZE>> DerefMut for Stack<STACK_SIZE, IS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const STACK_SIZE: usize, IS: InstructionSet<STACK_SIZE>> Default for Stack<STACK_SIZE, IS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const STACK_SIZE: usize, IS: InstructionSet<STACK_SIZE>> Stack<STACK_SIZE, IS> {
    /// Create a new stack with all elements initialized to the default value.
    #[must_use]
    pub fn new() -> Self {
        Self([IS::W::default(); STACK_SIZE])
    }

    /// Read a value from the stack at the given stack pointer.
    /// Returns the value on the stack or an `OutOfBounds` error.
    pub fn read(&self, sp: usize) -> Result<&IS::W, StackError> {
        self.get(sp).ok_or(StackError::OutOfBounds {
            sp,
            stack_size: self.len(),
        })
    }

    /// Write a value to the stack at the given stack pointer.
    /// Returns an `OutOfBounds` error if the stack pointer is out of bounds.
    pub fn write(&mut self, sp: usize, value: IS::W) -> Result<(), StackError> {
        let stack_size = self.len();

        self.get_mut(sp).map_or_else(
            || Err(StackError::OutOfBounds { sp, stack_size }),
            |adr| {
                *adr = value;
                Ok(())
            },
        )
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum StackError {
    #[error("Out of bounds stack access. Stack size: {stack_size}, Stack pointer: {sp}")]
    OutOfBounds { sp: usize, stack_size: usize },
}
