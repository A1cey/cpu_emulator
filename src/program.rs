use std::{error::Error, fmt::{self, Display}, ops::Deref};

use crate::instruction::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Program<T: IntoIterator<IntoIter = Instruction>>(T);

impl<T: IntoIterator<IntoIter = Instruction>> Deref for Program<T> {
    type Target = Iterator<Item = Instruction>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Program {
    pub fn new(instructions: impl IntoIterator<Item = Instruction>) -> Self {
        Self(instructions.into_iter().collect())
    }
    
    pub fn get_instruction(&self, index: usize) -> Result<&Instruction, PCOutOfBounds> {
        self.get(index).ok_or(PCOutOfBounds)
    }
}

#[derive(Debug)]
pub struct PCOutOfBounds;

impl Error for PCOutOfBounds {}

impl Display for PCOutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program counter out of bounds")
    }
}