use core::marker::PhantomData;
use core::ops::Deref;
use thiserror::Error;

use crate::instruction::Instruction;
use crate::word::Word;

#[derive(Debug, Clone, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
pub struct Program<I, T, W>(T, PhantomData<(I, W)>);

impl<T, I, W: Word> Deref for Program<I, T, W>
where
    I: Instruction<W>,
    T: Deref<Target = [I]>,
{
    type Target = [I];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I, T, W> From<T> for Program<I, T, W>
where
    I: Instruction<W>,
    T: Deref<Target = [I]>,
    W: Word,
{
    fn from(instructions: T) -> Self {
        Self(instructions, PhantomData)
    }
}

impl<T, I, W> Program<I, T, W>
where
    I: Instruction<W>,
    T: Deref<Target = [I]>,
    W: Word,
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
    pub fn fetch_instruction(&self, pc: usize) -> Result<I, ProgramError> {
        self.get(pc).map_or_else(
            || {
                Err(ProgramError::PCOutOfBounds {
                    pc,
                    program_len: self.len(),
                })
            },
            |instruction| Ok(*instruction),
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
