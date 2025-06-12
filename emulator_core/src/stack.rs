use core::fmt::Debug;
use core::ops::{Deref, DerefMut};
use thiserror::Error;

use crate::instruction_set::InstructionSet;

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
