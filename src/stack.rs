use core::fmt::{Debug, Display};
use core::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use thiserror::Error;

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

// /// This macro is used to implement From<_> for usize for the Word variants.
// macro_rules! from_word_for_usize {
//     ($name: ident $(,)? ) => {
//         impl ::core::convert::From<$name> for usize {
//             fn from(value: $name) -> usize {
//                 value.0 as usize
//             }
//         }
//     };
// }

/// This macro can be used to implement the Word trait for a Wrapper struct around another type like u8.
macro_rules! impl_word {
    ($name: ident, $type: ty $(,)? ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[repr(transparent)]
        pub struct $name($type);

        impl Word for $name {}

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
                *self = Self(self.0 + other.0);
            }
        }

        impl ::core::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                *self = Self(self.0 - other.0);
            }
        }

        impl ::core::ops::MulAssign for $name {
            fn mul_assign(&mut self, other: Self) {
                *self = Self(self.0 * other.0);
            }
        }

        impl ::core::ops::DivAssign for $name {
            fn div_assign(&mut self, other: Self) {
                *self = Self(self.0 / other.0);
            }
        }

        impl ::core::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl ::core::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl ::core::ops::Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                Self(self.0 * rhs.0)
            }
        }

        impl ::core::ops::Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                Self(self.0 / rhs.0)
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

// from_word_for_usize!(U8);
// from_word_for_usize!(U16);
// from_word_for_usize!(U32);
// from_word_for_usize!(U64);
// from_word_for_usize!(U128);
// from_word_for_usize!(USize);
// from_word_for_usize!(I8);
// from_word_for_usize!(I16);
// from_word_for_usize!(I64);
// from_word_for_usize!(I128);
// from_word_for_usize!(ISize);

/// Stack
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Stack<T: Word, const SIZE: usize>(pub [T; SIZE]);

impl<T: Word, const SIZE: usize> Deref for Stack<T, SIZE> {
    type Target = [T; SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Word, const SIZE: usize> DerefMut for Stack<T, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Word, const SIZE: usize> Default for Stack<T, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Word, const SIZE: usize> Stack<T, SIZE> {
    /// Create a new stack with all elements initialized to the default value.
    pub fn new() -> Self {
        Self([T::default(); SIZE])
    }

    /// Read a value from the stack at the given stack pointer.
    /// Returns the value on the stack or an `OutOfBounds` error.
    fn read(&self, sp: usize) -> Result<&T, StackError> {
        self.get(sp).ok_or(StackError::OutOfBounds {
            sp,
            stack_size: self.len(),
        })
    }

    /// Write a value to the stack at the given stack pointer.
    /// Returns an `OutOfBounds` error if the stack pointer is out of bounds.
    fn write(&mut self, sp: usize, value: T) -> Result<(), StackError> {
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

#[derive(Error, Debug)]
pub enum StackError {
    #[error("Out of bounds stack access. Stack size: {stack_size}, Stack pointer: {sp}")]
    OutOfBounds { sp: usize, stack_size: usize },
}
