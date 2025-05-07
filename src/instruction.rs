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
    /// End of program
    End,
    /// Move value from one register to another
    MoveReg { to: Register, from: Register },
    /// Move value into register
    MoveVal {
        to: Register,
        val: RegisterValue<R, W>,
    },
    /// Add the value of two registers and store it in the first
    AddReg { acc: Register, rhs: Register },
    /// Add a value to a registers and store it in the register
    AddVal {
        acc: Register,
        val: RegisterValue<R, W>,
    },
    /// Subtract the value of two registers and store it in the first
    SubReg { acc: Register, rhs: Register },
    /// Subtract a value to a registers and store it in the register
    SubVal {
        acc: Register,
        val: RegisterValue<R, W>,
    },
    /// Multiply the value of two registers and store it in the first
    MulReg { acc: Register, rhs: Register },
    /// Multiply a value to a registers and store it in the register
    MulVal {
        acc: Register,
        val: RegisterValue<R, W>,
    },
    /// Divide the value of two registers and store it in the first
    DivReg { acc: Register, rhs: Register },
    /// Divide a value to a registers and store it in the register
    DivVal {
        acc: Register,
        val: RegisterValue<R, W>,
    },
    /// Increase the value in a register by one
    Inc { reg: Register },
    /// Decrease the value in a register by one
    Dec { reg: Register },
}

impl<R: RegisterSize, W: Word, const STACK_SIZE: usize> Instruction<R, W, STACK_SIZE> {
    pub fn execute(
        instruction: &Instruction<R, W, STACK_SIZE>,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), ExecutionError> {
        match instruction {
            Instruction::Nop => Ok(()),
            Instruction::End => Ok(()),
            Instruction::MoveReg { to, from } => Self::move_reg(*to, *from, processor),
            Instruction::MoveVal { to, val } => Self::move_val(*to, *val, processor),
            Instruction::AddReg { acc, rhs } => Self::add_reg(*acc, *rhs, processor),
            Instruction::AddVal { acc, val } => Self::add_val(*acc, *val, processor),
            Instruction::SubReg { acc, rhs } => Self::sub_reg(*acc, *rhs, processor),
            Instruction::SubVal { acc, val } => Self::sub_val(*acc, *val, processor),
            Instruction::MulReg { acc, rhs } => Self::mul_reg(*acc, *rhs, processor),
            Instruction::MulVal { acc, val } => Self::mul_val(*acc, *val, processor),
            Instruction::DivReg { acc, rhs } => Self::div_reg(*acc, *rhs, processor),
            Instruction::DivVal { acc, val } => Self::div_val(*acc, *val, processor),
            Instruction::Inc { reg } => Ok(Self::inc(*reg, processor)),
            Instruction::Dec { reg } => Ok(Self::dec(*reg, processor)),
        }
        .map_err(Into::into)
    }

    fn move_reg(
        to: Register,
        from: Register,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        let val = processor.registers.get(from);
        Self::move_val(to, val, processor)
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
        Self::add_val(acc, val, processor)
    }

    fn add_val(
        acc: Register,
        val: RegisterValue<R, W>,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        processor.registers.add(acc, val)
    }

    fn sub_reg(
        acc: Register,
        rhs: Register,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        let val = processor.registers.get(rhs);
        Self::sub_val(acc, val, processor)
    }

    fn sub_val(
        acc: Register,
        val: RegisterValue<R, W>,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        processor.registers.sub(acc, val)
    }

    fn mul_reg(
        acc: Register,
        rhs: Register,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        let val = processor.registers.get(rhs);
        Self::mul_val(acc, val, processor)
    }

    fn mul_val(
        acc: Register,
        val: RegisterValue<R, W>,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        processor.registers.mul(acc, val)
    }

    fn div_reg(
        acc: Register,
        rhs: Register,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        let val = processor.registers.get(rhs);
        Self::div_val(acc, val, processor)
    }

    fn div_val(
        acc: Register,
        val: RegisterValue<R, W>,
        processor: &mut Processor<R, W, STACK_SIZE>,
    ) -> Result<(), RegisterError> {
        processor.registers.div(acc, val)
    }

    fn inc(register: Register, processor: &mut Processor<R, W, STACK_SIZE>) -> () {
        processor.registers.inc(register)
    }

    fn dec(register: Register, processor: &mut Processor<R, W, STACK_SIZE>) -> () {
        processor.registers.dec(register)
    }
}

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("register error")]
    Register(#[from] RegisterError),
}
