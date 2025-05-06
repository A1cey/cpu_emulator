use core::ops::{Deref, DerefMut};
use thiserror::Error;

/// Marker trait for types that can be used as words in a stack.
pub trait Word: Copy + Default + Sized + Into<usize> {}

macro_rules! impl_word {
    ($name: ident, $type: ty $(,)? ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[repr(transparent)]
        pub struct $name($type);

        impl Word for $name {}

        impl ::core::convert::Into<usize> for $name {
            fn into(self) -> usize {
                self.0 as usize
            }
        }

        impl ::core::convert::From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }
    };
}

impl_word!(U16, u16);
impl_word!(U32, u32);
impl_word!(U64, u64);
impl_word!(USize, usize);

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

impl<T: Word, const SIZE: usize> Stack<T, SIZE> {
    /// Create a new stack with all elements initialized to the default value.
    pub fn new() -> Self {
        Self([T::default(); SIZE])
    }

    /// Read a value from the stack at the given stack pointer.
    /// Returns the value on the stack or an OutOfBounds error.
    fn read(&self, sp: usize) -> Result<&T, StackError> {
        self.get(sp).ok_or(StackError::OutOfBounds {
            sp,
            stack_size: self.len(),
        })
    }

    /// Write a value to the stack at the given stack pointer.
    /// Returns an OutOfBounds error if the stack pointer is out of bounds.
    fn write(&mut self, sp: usize, value: T) -> Result<(), StackError> {
        match self.get_mut(sp) {
            Some(adr) => {
                *adr = value;
                Ok(())
            }
            None => Err(StackError::OutOfBounds {
                sp,
                stack_size: self.len(),
            }),
        }
    }
}

#[derive(Error, Debug)]
pub enum StackError {
    #[error("Out of bounds stack access. Stack size: {stack_size}, Stack pointer: {sp}")]
    OutOfBounds { sp: usize, stack_size: usize },
}
