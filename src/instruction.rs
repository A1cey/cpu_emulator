use thiserror::Error;

use crate::{
    processor::Processor,
    register::{Register, RegisterError, RegisterSize, RegisterValue},
    stack::Word,
};

// TODO: Instruction set trait to implement different instruction sets

/// Instruction set for the processor
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction<R: RegisterSize, W: Word, const STACK_SIZE: usize> {
    /// No operation
    Nop,
    /// Move value from one register to another
    MoveReg { to: Register, from: Register },
    /// Move value into register
    MoveVal {
        to: Register,
        val: RegisterValue<R, W>,
    },
    /// Add the value of two registers and store it in the first
    AddReg { acc: Register, rhs: Register },
}

impl<R: RegisterSize, W: Word, const STACK_SIZE: usize> Instruction<R, W, STACK_SIZE> {
    pub fn execute(
        instruction: &Instruction<R, W, STACK_SIZE>,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), ExecutionError> {
        match instruction {
            Instruction::Nop => Ok(()),
            Instruction::MoveReg { to, from } => Self::move_reg(*to, *from, processor),
            Instruction::MoveVal { to, val } => Self::move_val(*to, *val, processor),
            Instruction::AddReg { acc, rhs } => Self::add_reg(*acc, *rhs, processor),
        }
        .map_err(Into::into)
    }

    fn move_reg(
        to: Register,
        from: Register,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        let val = processor.registers.get(from);
        processor.registers.set(to, val)
    }

    fn move_val(
        to: Register,
        val: RegisterValue<R, W>,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        processor.registers.set(to, val)
    }

    fn add_reg(
        acc: Register,
        rhs: Register,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        let val = processor.registers.get(rhs);
        processor.registers.add(acc, val)
    }
}

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("register error")]
    Register(#[from] RegisterError),
}
