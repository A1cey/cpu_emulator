use core::ops::Deref;
use thiserror::Error;

use crate::instruction::{InstructionSet};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Program<IS: InstructionSet>(Vec<IS::Instruction>);

impl< IS: InstructionSet> Deref for Program<IS> {
    type Target = Vec<IS::Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<IS: InstructionSet> Program<IS> {
    pub fn new(instructions: impl IntoIterator<Item = IS::Instruction>) -> Self {
        Self(instructions.into_iter().collect())
    }

    /// Returns instruction at the provided index.
    ///
    /// # Errors
    /// Returns `PCOutOfBounds` error if the program counter is not in bounds.
    pub fn get_instruction(&self, pc: usize) -> Result<&IS::Instruction, ProgramError> {
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
