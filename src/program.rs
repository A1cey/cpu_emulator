use core::ops::Deref;
use thiserror::Error;

use crate::instruction::Instruction;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Program(Vec<Instruction>);

impl Deref for Program {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Program {
    pub fn new(instructions: impl IntoIterator<Item = Instruction>) -> Self {
        Self(instructions.into_iter().collect())
    }

    pub fn get_instruction(&self, index: usize) -> Result<&Instruction, ProgramError> {
        self.get(index).ok_or(ProgramError::PCOutOfBounds {
            pc: index,
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
