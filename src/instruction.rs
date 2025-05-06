use thiserror::Error;

use crate::{
    processor::Processor,
    register::{Register, RegisterError, RegisterSize},
    stack::Word,
};

// TODO: Instruction set trait to implement different instruction sets

/// Instruction set for the processor
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    /// No operation
    Nop,
    /// Move value from one register to another
    MoveReg { to: Register, from: Register },
}

impl Instruction {
    pub fn execute<R: RegisterSize, W: Word, const STACK_SIZE: usize>(
        instruction: &Instruction,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        match instruction {
            Instruction::Nop => Ok(()),
            Instruction::MoveReg { to, from } => {
                Self::move_reg::<R, W, STACK_SIZE>(*to, *from, processor)
            }
        }
    }

    fn move_reg<R: RegisterSize, W: Word, const STACK_SIZE: usize>(
        to: Register,
        from: Register,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        let val = processor.registers.get(from);
        processor.registers.set(to, val)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ExecutionError {}
