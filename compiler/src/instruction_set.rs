use emulator_core::instruction_set::{InstructionSet};
use emulator_core::processor::Processor;
use emulator_core::register::{Register};
use emulator_core::stack::Word;

use core::ops::ControlFlow;

/// Default instruction set for the processor.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction<const STACK_SIZE: usize, W: Word> {
    /// No operation. [NOP]
    Nop,
    /// End of program. [END]
    End,
    /// Copy a value from one register to another register. [MOV]
    MoveReg { to: Register, from: Register },
    /// Copy a value into a register. [MOV]
    MoveVal { to: Register, val: W },
    /// Add the value of a register (rhs) to another register (acc). [ADD]
    AddReg { acc: Register, rhs: Register },
    /// Add a value to a register (acc). [ADD]
    AddVal { acc: Register, val: W },
    /// Subtract the value of a register (rhs) from another register (acc). [SUB]
    SubReg { acc: Register, rhs: Register },
    /// Subtract a value from a register (acc). [SUB]
    SubVal { acc: Register, val: W },
    /// Multiply the value of a register (rhs) with the value of another register (acc).
    /// The result is stored in acc. [MUL]
    MulReg { acc: Register, rhs: Register },
    /// Multiply a value to with the value of a register (acc).
    /// The result is stored in this register. [MUL]
    MulVal { acc: Register, val: W },
    /// Divide the value of a register (acc) by the value of another register (rhs).
    /// The result is stored in acc. [DIV]
    DivReg { acc: Register, rhs: Register },
    /// Divide the value of a register (acc) by another value.
    /// The result is stored in the register. [DIV]
    DivVal { acc: Register, val: W },
    /// Increment the value in a register by one. [INC]
    Inc { reg: Register },
    /// Decrement the value in a register by one. [DEC]
    Dec { reg: Register },
    /// Set program pointer to value, effectively jumping to the instruction at this point in the program. [JMP]
    Jump { to: W },
}

impl<const STACK_SIZE: usize, W: Word> InstructionSet<STACK_SIZE> for Instruction< STACK_SIZE, W> {
    type Instruction = Self;
    type W = W;

    /// Execute an instruction on a processor.
    fn execute(instruction: &Self, processor: &mut Processor<STACK_SIZE, Self>) -> ControlFlow<()> {
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
            Self::Jump { to } => Self::jmp(*to, processor),
        };

        ControlFlow::Continue(())
    }
}

impl<const STACK_SIZE: usize, W: Word> Instruction< STACK_SIZE, W> {
    /// Copy a value from a register to another register.
    #[inline]
    fn move_reg(to: Register, from: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(from);
        Self::move_val(to, val, processor);
    }

    /// Copy a value into a register.
    #[inline]
    fn move_val(to: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        processor.registers.set_reg(to, val);
    }

    /// Add the value of a register (rhs) to another register (acc).
    #[inline]
    fn add_reg(acc: Register, rhs: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(rhs);
        Self::add_val(acc, val, processor);
    }

    /// Add a value to a register (acc).
    #[inline]
    fn add_val(acc: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        let a = processor.registers.get_reg(acc);
        processor.registers.set_reg(acc, a + val);
    }

    /// Subtract the value of a register (rhs) from another register (acc).
    #[inline]
    fn sub_reg(acc: Register, rhs: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(rhs);
        Self::sub_val(acc, val, processor);
    }

    /// Subtract a value from a register (acc).
    #[inline]
    fn sub_val(acc: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        let a = processor.registers.get_reg(acc);
        processor.registers.set_reg(acc, a - val);
    }

    /// Multiply the value of a register (rhs) with the value of another register (acc).
    /// The result is stored in acc.
    #[inline]
    fn mul_reg(acc: Register, rhs: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(rhs);
        Self::mul_val(acc, val, processor);
    }

    /// Multiply a value to with the value of a register (acc).
    /// The result is stored in this register.
    #[inline]
    fn mul_val(acc: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        let a = processor.registers.get_reg(acc);
        processor.registers.set_reg(acc, a * val);
    }

    /// Divide the value of a register (acc) by the value of another register (rhs).
    /// The result is stored in acc.
    #[inline]
    fn div_reg(acc: Register, rhs: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(rhs);
        Self::div_val(acc, val, processor);
    }

    /// Divide the value of a register (acc) by another value.
    /// The result is stored in the register.
    #[inline]
    fn div_val(acc: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        let a = processor.registers.get_reg(acc);
        processor.registers.set_reg(acc, a / val);
    }

    /// Increment the value in a register by one.
    #[inline]
    fn inc(reg: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        processor.registers.inc(reg);
    }

    /// Decrement the value in a register by one.
    #[inline]
    fn dec(reg: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        processor.registers.dec(reg);
    }

    /// Set program counter to value, effectively jumping to the instruction at this point in the program.
    #[inline]
    fn jmp(to: W, processor: &mut Processor<STACK_SIZE, Self>) {
        processor.registers.set_reg(Register::PC, to);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use emulator_core::stack::*;

    const STACK_SIZE: usize = 32;
    type IS = Instruction< STACK_SIZE, I8>;

    #[test]
    fn test_move_reg() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        let _ = IS::execute(
            &Instruction::MoveReg {
                from: Register::R0,
                to: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(
            processor.registers.get_reg(Register::R1),
            processor.registers.get_reg(Register::R0)
        );
    }

    #[test]
    fn test_move_val() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        let _ = IS::execute(
            &Instruction::MoveVal {
                to: Register::R0,
                val: 10.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 10.into());
    }

    #[test]
    fn test_inc() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        let _ = IS::execute(&Instruction::Inc { reg: Register::R0 }, &mut processor);
        assert_eq!(processor.registers.get_reg(Register::R0), 11.into());
    }

    #[test]
    fn test_inc_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MAX.into());
        let _ = IS::execute(&Instruction::Inc { reg: Register::R0 }, &mut processor);
        assert_eq!(processor.registers.get_reg(Register::R0), i8::MIN.into());
    }

    #[test]
    fn test_dec() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        let _ = IS::execute(&Instruction::Dec { reg: Register::R0 }, &mut processor);
        assert_eq!(processor.registers.get_reg(Register::R0), 9.into());
    }

    #[test]
    fn test_dec_underflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MIN.into());
        let _ = IS::execute(&Instruction::Dec { reg: Register::R0 }, &mut processor);
        assert_eq!(processor.registers.get_reg(Register::R0), i8::MAX.into());
    }

    #[test]
    fn test_add_reg() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        processor.registers.set_reg(Register::R1, 10.into());
        let _ = IS::execute(
            &Instruction::AddReg {
                acc: Register::R0,
                rhs: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 15.into());
    }

    #[test]
    fn test_add_reg_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MAX.into());
        processor.registers.set_reg(Register::R1, 1.into());
        let _ = IS::execute(
            &Instruction::AddReg {
                acc: Register::R0,
                rhs: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), i8::MIN.into());
    }

    #[test]
    fn test_add_val() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        let _ = IS::execute(
            &Instruction::AddVal {
                acc: Register::R0,
                val: 10.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 15.into());
    }

    #[test]
    fn test_add_val_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MAX.into());
        let _ = IS::execute(
            &Instruction::AddVal {
                acc: Register::R0,
                val: 1.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), i8::MIN.into());
    }

    #[test]
    fn test_sub_reg() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        processor.registers.set_reg(Register::R1, 10.into());
        let _ = IS::execute(
            &Instruction::SubReg {
                acc: Register::R0,
                rhs: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-5).into());
    }

    #[test]
    fn test_sub_reg_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MIN.into());
        processor.registers.set_reg(Register::R1, 1.into());
        let _ = IS::execute(
            &Instruction::SubReg {
                acc: Register::R0,
                rhs: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), i8::MAX.into());
    }

    #[test]
    fn test_sub_val() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        let _ = IS::execute(
            &Instruction::SubVal {
                acc: Register::R0,
                val: 10.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-5).into());
    }

    #[test]
    fn test_sub_val_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, (-128).into());
        let _ = IS::execute(
            &Instruction::SubVal {
                acc: Register::R0,
                val: 1.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 127.into());
    }

    #[test]
    fn test_mul_reg() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        processor.registers.set_reg(Register::R1, 10.into());
        let _ = IS::execute(
            &Instruction::MulReg {
                acc: Register::R0,
                rhs: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 50.into());

        processor.registers.set_reg(Register::R0, (-5).into());
        processor.registers.set_reg(Register::R1, 10.into());
        let _ = IS::execute(
            &Instruction::MulReg {
                acc: Register::R0,
                rhs: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-50).into());
    }

    #[test]
    fn test_mul_reg_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 80.into());
        processor.registers.set_reg(Register::R1, 2.into());
        let _ = IS::execute(
            &Instruction::MulReg {
                acc: Register::R0,
                rhs: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-96).into());
    }

    #[test]
    fn test_mul_reg_underflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, (-80).into());
        processor.registers.set_reg(Register::R1, 2.into());
        let _ = IS::execute(
            &Instruction::MulReg {
                acc: Register::R0,
                rhs: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 96.into());
    }

    #[test]
    fn test_mul_val() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        let _ = IS::execute(
            &Instruction::MulVal {
                acc: Register::R0,
                val: 10.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 50.into());

        processor.registers.set_reg(Register::R0, (-5).into());
        let _ = IS::execute(
            &Instruction::MulVal {
                acc: Register::R0,
                val: 10.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-50).into());
    }

    #[test]
    fn test_mul_val_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 80.into());
        let _ = IS::execute(
            &Instruction::MulVal {
                acc: Register::R0,
                val: 2.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-96).into());
    }

    #[test]
    fn test_mul_val_underflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, (-80).into());
        let _ = IS::execute(
            &Instruction::MulVal {
                acc: Register::R0,
                val: 2.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 96.into());
    }

    #[test]
    fn test_div_reg() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        processor.registers.set_reg(Register::R1, 5.into());
        let _ = IS::execute(
            &Instruction::DivReg {
                acc: Register::R0,
                rhs: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 2.into());

        processor.registers.set_reg(Register::R0, (-10).into());
        processor.registers.set_reg(Register::R1, 5.into());
        let _ = IS::execute(
            &Instruction::DivReg {
                acc: Register::R0,
                rhs: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-2).into());
    }

    #[test]
    fn test_div_reg_truncate() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 3.into());
        processor.registers.set_reg(Register::R1, 2.into());
        let _ = IS::execute(
            &Instruction::DivReg {
                acc: Register::R0,
                rhs: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 1.into());
    }

    #[test]
    fn test_div_reg_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MIN.into());
        processor.registers.set_reg(Register::R1, (-1).into());
        let _ = IS::execute(
            &Instruction::DivReg {
                acc: Register::R0,
                rhs: Register::R1,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (i8::MIN).into());
    }

    #[test]
    fn test_div_val() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        let _ = IS::execute(
            &Instruction::DivVal {
                acc: Register::R0,
                val: 5.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 2.into());

        processor.registers.set_reg(Register::R0, (-10).into());
        let _ = IS::execute(
            &Instruction::DivVal {
                acc: Register::R0,
                val: 5.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-2).into());
    }

    #[test]
    fn test_div_val_truncate() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 3.into());
        let _ = IS::execute(
            &Instruction::DivVal {
                acc: Register::R0,
                val: 4.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 0.into());

        processor.registers.set_reg(Register::R0, 3.into());
        let _ = IS::execute(
            &Instruction::DivVal {
                acc: Register::R0,
                val: 2.into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 1.into());
    }

    #[test]
    fn test_div_val_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MIN.into());
        let _ = IS::execute(
            &Instruction::DivVal {
                acc: Register::R0,
                val: (-1).into(),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (i8::MIN).into());
    }

    #[test]
    fn test_jmp() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        assert_eq!(processor.registers.get_reg(Register::PC), 0.into());
        let _ = IS::execute(&Instruction::Jump { to: 2.into() }, &mut processor);
        assert_eq!(processor.registers.get_reg(Register::PC), 2.into());
    }

    #[test]
    fn test_jmp_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        assert_eq!(processor.registers.get_reg(Register::PC), 0.into());
        let _ = IS::execute(&Instruction::Jump { to: i8::MAX.into() }, &mut processor);
        assert_eq!(processor.registers.get_reg(Register::PC), i8::MAX.into());
        let _ = IS::execute(&Instruction::Inc { reg: Register::PC }, &mut processor);
        assert_eq!(processor.registers.get_reg(Register::PC), i8::MIN.into());
    }

    #[test]
    fn test_jmp_underflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        assert_eq!(processor.registers.get_reg(Register::PC), 0.into());
        let _ = IS::execute(&Instruction::Jump { to: i8::MIN.into() }, &mut processor);
        assert_eq!(processor.registers.get_reg(Register::PC), i8::MIN.into());
        let _ = IS::execute(&Instruction::Dec { reg: Register::PC }, &mut processor);
        assert_eq!(processor.registers.get_reg(Register::PC), i8::MAX.into());
    }
}
