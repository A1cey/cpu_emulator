use core::ops::ControlFlow;

use thiserror::Error;

use crate::{
    processor::Processor,
    register::{Register, RegisterError},
    stack::Word,
};

// TODO: Instruction set trait to implement different instruction sets

/// Instruction set for the processor
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction<W: Word, const STACK_SIZE: usize> {
    /// No operation.
    Nop,
    /// End of program.
    End,
    /// Copy a value from one register to another register.
    MoveReg { to: Register, from: Register },
    /// Copy a value into a register.
    MoveVal { to: Register, val: W },
    /// Add the value of a register (rhs) to another register (acc).
    AddReg { acc: Register, rhs: Register },
    /// Add a value to a register (acc).
    AddVal { acc: Register, val: W },
    /// Subtract the value of a register (rhs) from another register (acc).
    SubReg { acc: Register, rhs: Register },
    /// Subtract a value from a register (acc).
    SubVal { acc: Register, val: W },
    /// Multiply the value of a register (rhs) with the value of another register (acc).
    /// The result is stored in acc.
    MulReg { acc: Register, rhs: Register },
    /// Multiply a value to with the value of a register (acc).
    /// The result is stored in this register.
    MulVal { acc: Register, val: W },
    /// Divide the value of a register (acc) by the value of another register (rhs).
    /// The result is stored in acc.
    DivReg { acc: Register, rhs: Register },
    /// Divide the value of a register (acc) by another value.
    /// The result is stored in the register.
    DivVal { acc: Register, val: W },
    /// Increment the value in a register by one.
    Inc { reg: Register },
    /// Decrement the value in a register by one.
    Dec { reg: Register },
    /// Set program pointer to value, effectively jumping to the instruction at this point in the program.
    Jump {to: W}
}

impl<W: Word, const STACK_SIZE: usize> Instruction<W, STACK_SIZE> {
    /// Execute an instruction on a processor.
    pub fn execute(
        instruction: &Self,
        processor: &mut Processor<W, STACK_SIZE>,
    ) -> ControlFlow<()> {
        match instruction {
            Self::End => return ControlFlow::Break(()),
            Self::Nop => (),
            Self::MoveReg { to, from } => Self::move_reg(*to, *from, processor),
            Self::MoveVal { to, val } => Self::move_val(*to, *val, processor),
            Self::AddReg { acc, rhs } => Self::add_reg(*acc, *rhs, processor),
            Self::AddVal { acc, val } => Self::add_val(*acc, *val, processor),
            Self::SubReg { acc, rhs } => Self::sub_reg(*acc, *rhs, processor),
            Self::SubVal { acc, val } => Self::sub_val(*acc, *val, processor),
            Self::MulReg { acc, rhs } => Self::mul_reg(*acc, *rhs, processor),
            Self::MulVal { acc, val } => Self::mul_val(*acc, *val, processor),
            Self::DivReg { acc, rhs } => Self::div_reg(*acc, *rhs, processor),
            Self::DivVal { acc, val } => Self::div_val(*acc, *val, processor),
            Self::Inc { reg } => Self::inc(*reg, processor),
            Self::Dec { reg } => Self::dec(*reg, processor),
            Self::Jump { to } => Self::jmp(*to, processor)
        };

        ControlFlow::Continue(())
    }

    /// Copy a value from a register to another register.
    #[inline]
    fn move_reg(to: Register, from: Register, processor: &mut Processor<W, STACK_SIZE>) {
        let val = processor.registers.get(from);
        Self::move_val(to, val, processor);
    }

    /// Copy a value into a register.
    #[inline]
    fn move_val(to: Register, val: W, processor: &mut Processor<W, STACK_SIZE>) {
        processor.registers.set(to, val);
    }

    /// Add the value of a register (rhs) to another register (acc).
    #[inline]
    fn add_reg(acc: Register, rhs: Register, processor: &mut Processor<W, STACK_SIZE>) {
        let val = processor.registers.get(rhs);
        Self::add_val(acc, val, processor);
    }

    /// Add a value to a register (acc).
    #[inline]
    fn add_val(acc: Register, val: W, processor: &mut Processor<W, STACK_SIZE>) {
        let a = processor.registers.get(acc);
        processor.registers.set(acc, a + val);
    }

    /// Subtract the value of a register (rhs) from another register (acc).
    #[inline]
    fn sub_reg(acc: Register, rhs: Register, processor: &mut Processor<W, STACK_SIZE>) {
        let val = processor.registers.get(rhs);
        Self::sub_val(acc, val, processor);
    }

    /// Subtract a value from a register (acc).
    #[inline]
    fn sub_val(acc: Register, val: W, processor: &mut Processor<W, STACK_SIZE>) {
        let a = processor.registers.get(acc);
        processor.registers.set(acc, a - val);
    }

    /// Multiply the value of a register (rhs) with the value of another register (acc).
    /// The result is stored in acc.
    #[inline]
    fn mul_reg(acc: Register, rhs: Register, processor: &mut Processor<W, STACK_SIZE>) {
        let val = processor.registers.get(rhs);
        Self::mul_val(acc, val, processor);
    }

    /// Multiply a value to with the value of a register (acc).
    /// The result is stored in this register.
    #[inline]
    fn mul_val(acc: Register, val: W, processor: &mut Processor<W, STACK_SIZE>) {
        let a = processor.registers.get(acc);
        processor.registers.set(acc, a * val);
    }

    /// Divide the value of a register (acc) by the value of another register (rhs).
    /// The result is stored in acc.
    #[inline]
    fn div_reg(acc: Register, rhs: Register, processor: &mut Processor<W, STACK_SIZE>) {
        let val = processor.registers.get(rhs);
        Self::div_val(acc, val, processor);
    }

    /// Divide the value of a register (acc) by another value.
    /// The result is stored in the register.
    #[inline]
    fn div_val(acc: Register, val: W, processor: &mut Processor<W, STACK_SIZE>) {
        let a = processor.registers.get(acc);
        processor.registers.set(acc, a / val);
    }

    /// Increment the value in a register by one.
    #[inline]
    fn inc(reg: Register, processor: &mut Processor<W, STACK_SIZE>) {
        processor.registers.inc(reg);
    }

    /// Decrement the value in a register by one.
    #[inline]
    fn dec(reg: Register, processor: &mut Processor<W, STACK_SIZE>) {
        processor.registers.dec(reg);
    }
    
    /// Set program counter to value, effectively jumping to the instruction at this point in the program.
    #[inline]
    fn jmp(to: W, processor: &mut Processor<W, STACK_SIZE>) {
        processor.registers.set(Register::PC, to);
    }
}

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("register error")]
    Register(#[from] RegisterError),
}
