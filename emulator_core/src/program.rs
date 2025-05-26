use core::ops::Deref;
use std::marker::PhantomData;
use thiserror::Error;

use crate::instruction_set::InstructionSet;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Program<const STACK_SIZE: usize, IS, P>(P, PhantomData<IS>)
where
    IS: InstructionSet<STACK_SIZE, P>,
    P: Deref<Target = [IS::Instruction]>;

impl<const STACK_SIZE: usize, IS, P> Deref for Program<STACK_SIZE, IS, P>
where
    IS: InstructionSet<STACK_SIZE, P>,
    P: Deref<Target = [IS::Instruction]>,
{
    type Target = [IS::Instruction];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const STACK_SIZE: usize, IS, P> Program<STACK_SIZE, IS, P>
where
    IS: InstructionSet<STACK_SIZE, P>,
    P: Deref<Target = [IS::Instruction]>,
{
    pub fn new(instructions: P) -> Self {
        Self(instructions, PhantomData)
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
