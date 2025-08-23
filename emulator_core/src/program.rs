use core::ops::Deref;
use std::marker::PhantomData;
use thiserror::Error;

use crate::instruction_set::InstructionSet;

#[derive(Debug, Clone, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
pub struct Program<IS, T>(T, PhantomData<IS>);

impl<T, IS> Deref for Program<IS, T>
where
    IS: InstructionSet,
    T: Deref<Target = [IS::Instruction]>,
{
    type Target = [IS::Instruction];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<IS, T> From<T> for Program<IS, T>
where
    IS: InstructionSet,
    T: Deref<Target = [IS::Instruction]>,
{
    fn from(instructions: T) -> Self {
        Self(instructions, PhantomData)
    }
}

impl<T, IS> Program<IS, T>
where
    IS: InstructionSet,
    T: Deref<Target = [IS::Instruction]>,
{
    /// Creates a new program from the provided instructions.
    #[must_use]
    pub fn new(instructions: T) -> Self {
        instructions.into()
    }

    /// Returns the instruction at the provided index.
    ///
    /// # Errors
    /// Returns `PCOutOfBounds` error if the program counter is not in bounds.
    #[inline]
    pub fn fetch_instruction(&self, pc: usize) -> Result<IS::Instruction, ProgramError> {
        self.get(pc).map_or_else(
            || {
                Err(ProgramError::PCOutOfBounds {
                    pc,
                    program_len: self.len(),
                })
            },
            |instruction| Ok(instruction.to_owned()),
        )
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ProgramError {
    #[error("Program counter out of bounds. Program length: {program_len}, Program counter: {pc}")]
    PCOutOfBounds { pc: usize, program_len: usize },
    #[error("No program loaded")]
    NoProgramLoaded,
}
