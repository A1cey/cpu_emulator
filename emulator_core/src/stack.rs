use crate::helper;
use crate::word::Word;
use core::fmt::{Debug, Display, Formatter};
use core::ops::{Deref, DerefMut};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Stack<const STACK_SIZE: usize, W: Word>(pub [W; STACK_SIZE]);

impl<const STACK_SIZE: usize, W: Word> Deref for Stack<STACK_SIZE, W> {
    type Target = [W; STACK_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const STACK_SIZE: usize, W: Word> DerefMut for Stack<STACK_SIZE, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const STACK_SIZE: usize, W: Word> Default for Stack<STACK_SIZE, W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const STACK_SIZE: usize, W: Word> Display for Stack<STACK_SIZE, W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{}", helper::FmtArray(self.deref().as_slice()))
    }
}

impl<const STACK_SIZE: usize, W: Word> Stack<STACK_SIZE, W> {
    /// Create a new stack with all elements initialized to the default value.
    #[must_use]
    pub fn new() -> Self {
        Self([W::default(); STACK_SIZE])
    }

    /// Read a value from the stack at the given stack pointer.
    ///
    /// # Errors
    /// Returns the value on the stack or an `OutOfBounds` error.
    pub fn read(&self, sp: usize) -> Result<&W, StackError> {
        self.get(sp).ok_or(StackError::OutOfBounds {
            sp,
            stack_size: self.len(),
        })
    }

    /// Write a value to the stack at the given stack pointer.
    ///
    /// # Errors
    /// Returns an `OutOfBounds` error if the stack pointer is out of bounds.
    pub fn write(&mut self, sp: usize, value: W) -> Result<(), StackError> {
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
