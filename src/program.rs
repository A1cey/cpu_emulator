use core::ops::Deref;
use thiserror::Error;

use crate::{instruction::Instruction, stack::Word};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Program<W: Word, const STACK_SIZE: usize>(Vec<Instruction<W, STACK_SIZE>>);

impl<W: Word, const STACK_SIZE: usize> Deref for Program<W, STACK_SIZE> {
    type Target = Vec<Instruction<W, STACK_SIZE>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<W: Word, const STACK_SIZE: usize> Program<W, STACK_SIZE> {
    pub fn new(instructions: impl IntoIterator<Item = Instruction<W, STACK_SIZE>>) -> Self {
        Self(instructions.into_iter().collect())
    }

    /// Returns instruction at the provided index.
    ///
    /// # Errors
    /// Returns `PCOutOfBounds` error if the program counter is not in bounds.
    pub fn get_instruction(&self, pc: usize) -> Result<&Instruction<W, STACK_SIZE>, ProgramError> {
        self.get(pc).ok_or(ProgramError::PCOutOfBounds {
            pc,
            program_len: self.len(),
        })
    }
}

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("Program counter out of bounds. Program length: {program_len}, Program counter: {pc}")]
    PCOutOfBounds { pc: usize, program_len: usize },
    #[error("No program loaded")]
    NoProgramLoaded,
}
