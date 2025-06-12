use core::cmp::Ordering;

use emulator_core::instruction_set::InstructionSet;
use emulator_core::processor::Processor;
use emulator_core::register::{Flag, Register};
use emulator_core::word::Word;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum ASMBinaryInstruction {
    Mov,
    Add,
    AddS,
    Sub,
    SubS,
    Mul,
    MulS,
    Div,
    DivS,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum ASMUnaryInstruction {
    Inc,
    IncS,
    Dec,
    DecS,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum ASMJumpInstruction {
    Jmp,
    JZ,
    JNZ,
    JC,
    JNC,
    JS,
    JNS,
    JG,
    JL,
    JGE,
    JLE,
}

/// Default instruction set for the processor.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Instruction<const STACK_SIZE: usize, W: Word> {
    /// No operation. \[NOP\]
    Nop,
    /// Copy a value from one register to another register. \[MOV\]
    MoveReg { to: Register, from: Register },
    /// Copy a value into a register. \[MOV\]
    MoveVal { to: Register, val: W },
    /// Add the value of a register (rhs) to another register (acc). \[ADD\]
    AddReg { acc: Register, rhs: Register },
    /// Add the value of a register (rhs) to another register (acc) and set flags. \[ADDS\]
    AddRegSigned { acc: Register, rhs: Register },
    /// Add a value to a register (acc). \[ADD\]
    AddVal { acc: Register, val: W },
    /// Add a value to a register (acc) and set flags. \[ADDS\]
    AddValSigned { acc: Register, val: W },
    /// Subtract the value of a register (rhs) from another register (acc). \[SUB\]
    SubReg { acc: Register, rhs: Register },
    /// Subtract the value of a register (rhs) from another register (acc) and set flags. \[SUBS\]
    SubRegSigned { acc: Register, rhs: Register },
    /// Subtract a value from a register (acc). \[SUB\]
    SubVal { acc: Register, val: W },
    /// Subtract a value from a register (acc) and set flags. \[SUBS\]
    SubValSigned { acc: Register, val: W },
    /// Multiply the value of a register (rhs) with the value of another register (acc).
    /// The result is stored in acc. \[MUL\]
    MulReg { acc: Register, rhs: Register },
    /// Multiply the value of a register (rhs) with the value of another register (acc) and set flags.
    /// The result is stored in acc. \[MULS\]
    MulRegSigned { acc: Register, rhs: Register },
    /// Multiply a value to with the value of a register (acc).
    /// The result is stored in this register. \[MUL\]
    MulVal { acc: Register, val: W },
    /// Multiply a value to with the value of a register (acc) and set flags.
    /// The result is stored in this register. \[MULS\]
    MulValSigned { acc: Register, val: W },
    /// Divide the value of a register (acc) by the value of another register (rhs).
    /// The result is stored in acc. \[DIV\]
    DivReg { acc: Register, rhs: Register },
    /// Divide the value of a register (acc) by the value of another register (rhs) and set flags.
    /// The result is stored in acc. \[DIVS\]
    DivRegSigned { acc: Register, rhs: Register },
    /// Divide the value of a register (acc) by another value.
    /// The result is stored in the register. \[DIV\]
    DivVal { acc: Register, val: W },
    /// Divide the value of a register (acc) by another value and set flags.
    /// The result is stored in the register. \[DIVS\]
    DivValSigned { acc: Register, val: W },
    /// Increment the value in a register by one. \[INC\]
    Inc { reg: Register },
    /// Increment the value in a register by one and set flags. \[INCS\]
    IncSigned { reg: Register },
    /// Decrement the value in a register by one. \[DEC\]
    Dec { reg: Register },
    /// Decrement the value in a register by one and set flags. \[DECS\]
    DecSigned { reg: Register },
    /// Set program pointer to value, effectively jumping to the instruction at this point in the program. \[JMP\]
    Jump { to: W },
    /// Analog to `Jump` but only if zero flag is set. \[JZ\]
    JumpZero { to: W },
    /// Analog to `Jump` but only if zero flag is not set. \[JNZ\]
    JumpNotZero { to: W },
    /// Analog to `Jump` but only if carry flag is set. \[JC\]
    JumpCarry { to: W },
    /// Analog to `Jump` but only if carry flag is not set. \[JNC\]
    JumpNotCarry { to: W },
    /// Analog to `Jump` but only if signed flag is set. \[JS\]
    JumpSigned { to: W },
    /// Analog to `Jump` but only if signed flag is not set. \[JNS\]
    JumpNotSigned { to: W },
    /// Analog to `Jump` but only if zero flag and signed flag are not set. \[JG\]
    JumpGreater { to: W },
    /// Analog to `Jump` but only if zero flag is not set and signed flag is set. \[JL\]
    JumpLess { to: W },
    /// Analog to `Jump` but only if zero flag is set or signed flag is not set. \[JGE\]
    JumpGreaterOrEq { to: W },
    /// Analog to `Jump` but only if zero flag or signed flag is set. \[JLE\]
    JumpLessOrEq { to: W },
}

impl<const STACK_SIZE: usize, W: Word> InstructionSet<STACK_SIZE> for Instruction<STACK_SIZE, W> {
    type Instruction = Self;
    type W = W;

    /// Execute an instruction on a processor.
    fn execute(instruction: &Self, processor: &mut Processor<STACK_SIZE, Self>) {
        match instruction {
            Self::Nop => (),
            Self::MoveReg { to, from } => Self::move_reg(*to, *from, processor),
            Self::MoveVal { to, val } => Self::move_val(*to, *val, processor),
            Self::AddReg { acc, rhs } => Self::add_reg(*acc, *rhs, processor),
            Self::AddRegSigned { acc, rhs } => Self::add_reg_signed(*acc, *rhs, processor),
            Self::AddVal { acc, val } => Self::add_val(*acc, *val, processor),
            Self::AddValSigned { acc, val } => Self::add_val_signed(*acc, *val, processor),
            Self::SubReg { acc, rhs } => Self::sub_reg(*acc, *rhs, processor),
            Self::SubRegSigned { acc, rhs } => Self::sub_reg_signed(*acc, *rhs, processor),
            Self::SubVal { acc, val } => Self::sub_val(*acc, *val, processor),
            Self::SubValSigned { acc, val } => Self::sub_val_signed(*acc, *val, processor),
            Self::MulReg { acc, rhs } => Self::mul_reg(*acc, *rhs, processor),
            Self::MulRegSigned { acc, rhs } => Self::mul_reg_signed(*acc, *rhs, processor),
            Self::MulVal { acc, val } => Self::mul_val(*acc, *val, processor),
            Self::MulValSigned { acc, val } => Self::mul_val_signed(*acc, *val, processor),
            Self::DivReg { acc, rhs } => Self::div_reg(*acc, *rhs, processor),
            Self::DivRegSigned { acc, rhs } => Self::div_reg_signed(*acc, *rhs, processor),
            Self::DivVal { acc, val } => Self::div_val(*acc, *val, processor),
            Self::DivValSigned { acc, val } => Self::div_val_signed(*acc, *val, processor),
            Self::Inc { reg } => Self::inc(*reg, processor),
            Self::IncSigned { reg } => Self::inc_signed(*reg, processor),
            Self::Dec { reg } => Self::dec(*reg, processor),
            Self::DecSigned { reg } => Self::dec_signed(*reg, processor),
            Self::Jump { to } => Self::jmp(*to, processor),
            Self::JumpZero { to } => Self::jmp_zero(*to, processor),
            Self::JumpNotZero { to } => Self::jmp_not_zero(*to, processor),
            Self::JumpCarry { to } => Self::jmp_carry(*to, processor),
            Self::JumpNotCarry { to } => Self::jmp_not_carry(*to, processor),
            Self::JumpSigned { to } => Self::jmp_signed(*to, processor),
            Self::JumpNotSigned { to } => Self::jmp_not_signed(*to, processor),
            Self::JumpGreater { to } => Self::jmp_greater(*to, processor),
            Self::JumpLess { to } => Self::jmp_less(*to, processor),
            Self::JumpGreaterOrEq { to } => Self::jmp_greater_or_eq(*to, processor),
            Self::JumpLessOrEq { to } => Self::jmp_less_or_eq(*to, processor),
        }
    }
}

impl<const STACK_SIZE: usize, W: Word> Instruction<STACK_SIZE, W> {
    pub(crate) const fn from_binary_reg_instr(
        instr: ASMBinaryInstruction,
        lhs: Register,
        rhs: Register,
    ) -> Self {
        match instr {
            ASMBinaryInstruction::Mov => Self::MoveReg { to: lhs, from: rhs },
            ASMBinaryInstruction::Add => Self::AddReg { acc: lhs, rhs },
            ASMBinaryInstruction::AddS => Self::AddRegSigned { acc: lhs, rhs },
            ASMBinaryInstruction::Sub => Self::SubReg { acc: lhs, rhs },
            ASMBinaryInstruction::SubS => Self::SubRegSigned { acc: lhs, rhs },
            ASMBinaryInstruction::Mul => Self::MulReg { acc: lhs, rhs },
            ASMBinaryInstruction::MulS => Self::MulRegSigned { acc: lhs, rhs },
            ASMBinaryInstruction::Div => Self::DivReg { acc: lhs, rhs },
            ASMBinaryInstruction::DivS => Self::DivRegSigned { acc: lhs, rhs },
        }
    }

    pub(crate) const fn from_binary_val_instr(
        instr: ASMBinaryInstruction,
        lhs: Register,
        val: W,
    ) -> Self {
        match instr {
            ASMBinaryInstruction::Mov => Self::MoveVal { to: lhs, val },
            ASMBinaryInstruction::Add => Self::AddVal { acc: lhs, val },
            ASMBinaryInstruction::AddS => Self::AddValSigned { acc: lhs, val },
            ASMBinaryInstruction::Sub => Self::SubVal { acc: lhs, val },
            ASMBinaryInstruction::SubS => Self::SubValSigned { acc: lhs, val },
            ASMBinaryInstruction::Mul => Self::MulVal { acc: lhs, val },
            ASMBinaryInstruction::MulS => Self::MulValSigned { acc: lhs, val },
            ASMBinaryInstruction::Div => Self::DivVal { acc: lhs, val },
            ASMBinaryInstruction::DivS => Self::DivValSigned { acc: lhs, val },
        }
    }

    pub(crate) const fn from_unary_instr(instr: ASMUnaryInstruction, reg: Register) -> Self {
        match instr {
            ASMUnaryInstruction::Inc => Self::Inc { reg },
            ASMUnaryInstruction::IncS => Self::IncSigned { reg },
            ASMUnaryInstruction::Dec => Self::Dec { reg },
            ASMUnaryInstruction::DecS => Self::DecSigned { reg },
        }
    }

    pub(crate) const fn from_jump_instr(instr: ASMJumpInstruction, dest: W) -> Self {
        match instr {
            ASMJumpInstruction::Jmp => Self::Jump { to: dest },
            ASMJumpInstruction::JZ => Self::JumpZero { to: dest },
            ASMJumpInstruction::JNZ => Self::JumpNotZero { to: dest },
            ASMJumpInstruction::JC => Self::JumpCarry { to: dest },
            ASMJumpInstruction::JNC => Self::JumpNotCarry { to: dest },
            ASMJumpInstruction::JS => Self::JumpSigned { to: dest },
            ASMJumpInstruction::JNS => Self::JumpNotSigned { to: dest },
            ASMJumpInstruction::JG => Self::JumpGreater { to: dest },
            ASMJumpInstruction::JL => Self::JumpLess { to: dest },
            ASMJumpInstruction::JGE => Self::JumpGreaterOrEq { to: dest },
            ASMJumpInstruction::JLE => Self::JumpLessOrEq { to: dest },
        }
    }

    /// Sets the signed and zero flags.
    #[inline]
    fn set_signed_zero_flags(val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        match val.cmp(&(0.into())) {
            Ordering::Less => {
                processor.registers.set_flag(Flag::S, true);
                processor.registers.set_flag(Flag::Z, false);
            }
            Ordering::Equal => {
                processor.registers.set_flag(Flag::S, false);
                processor.registers.set_flag(Flag::Z, true);
            }
            Ordering::Greater => {
                processor.registers.set_flag(Flag::S, false);
                processor.registers.set_flag(Flag::Z, false);
            }
        }
    }

    /// Copy a value from a register to another register.
    #[inline]
    const fn move_reg(to: Register, from: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(from);
        Self::move_val(to, val, processor);
    }

    /// Copy a value into a register.
    #[inline]
    const fn move_val(to: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        processor.registers.set_reg(to, val);
    }

    /// Add the value of a register (rhs) to another register (acc).
    #[inline]
    fn add_reg(acc: Register, rhs: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(rhs);
        Self::add_val(acc, val, processor);
    }

    /// Add the value of a register (rhs) to another register (acc) and set flags.
    #[inline]
    fn add_reg_signed(acc: Register, rhs: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(rhs);
        Self::add_val_signed(acc, val, processor);
    }

    /// Add a value to a register (acc).
    #[inline]
    fn add_val(acc: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        let a = processor.registers.get_reg(acc);
        processor.registers.set_reg(acc, a + val);
    }

    /// Add a value to a register (acc) and set flags.
    #[inline]
    fn add_val_signed(acc: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        let acc_val = processor.registers.get_reg(acc);
        let (res, overflow) = acc_val.overflowing_add(val);
        let carry = acc_val.carry_add(val);

        processor.registers.set_reg(acc, res);
        processor.registers.set_flag(Flag::V, overflow);
        processor.registers.set_flag(Flag::C, carry);

        Self::set_signed_zero_flags(res, processor);
    }

    /// Subtract the value of a register (rhs) from another register (acc).
    #[inline]
    fn sub_reg(acc: Register, rhs: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(rhs);
        Self::sub_val(acc, val, processor);
    }

    /// Subtract the value of a register (rhs) from another register (acc) and set flags.
    #[inline]
    fn sub_reg_signed(acc: Register, rhs: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(rhs);
        Self::sub_val_signed(acc, val, processor);
    }

    /// Subtract a value from a register (acc).
    #[inline]
    fn sub_val(acc: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        let a = processor.registers.get_reg(acc);
        processor.registers.set_reg(acc, a - val);
    }

    /// Subtract a value from a register (acc) and set flags.
    #[inline]
    fn sub_val_signed(acc: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        let acc_val = processor.registers.get_reg(acc);
        let (res, overflow) = acc_val.overflowing_sub(val);
        let carry = acc_val.carry_sub(val);

        processor.registers.set_reg(acc, res);
        processor.registers.set_flag(Flag::V, overflow);
        processor.registers.set_flag(Flag::C, carry);

        Self::set_signed_zero_flags(res, processor);
    }

    /// Multiply the value of a register (acc) with the value of another register (rhs).
    /// The result is stored in acc.
    #[inline]
    fn mul_reg(acc: Register, rhs: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(rhs);
        Self::mul_val(acc, val, processor);
    }

    /// Multiply the value of a register (acc) with the value of another register (rhs) and set flags.
    /// The result is stored in acc.
    #[inline]
    fn mul_reg_signed(acc: Register, rhs: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(rhs);
        Self::mul_val_signed(acc, val, processor);
    }

    /// Multiply the value of a register (acc) with another value.
    /// The result is stored in this register.
    #[inline]
    fn mul_val(acc: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        let a = processor.registers.get_reg(acc);
        processor.registers.set_reg(acc, a * val);
    }

    /// Multiply the value of a register (acc) with another value and set flags.
    /// The result is stored in this register.
    #[inline]
    fn mul_val_signed(acc: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        let acc_val = processor.registers.get_reg(acc);
        let (res, overflow) = acc_val.overflowing_mul(val);
        let carry = acc_val.carry_mul(val);

        processor.registers.set_reg(acc, res);
        processor.registers.set_flag(Flag::V, overflow);
        processor.registers.set_flag(Flag::C, carry);

        Self::set_signed_zero_flags(res, processor);
    }

    /// Divide the value of a register (acc) by the value of another register (rhs).
    /// The result is stored in acc.
    #[inline]
    fn div_reg(acc: Register, rhs: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(rhs);
        Self::div_val(acc, val, processor);
    }

    /// Divide the value of a register (acc) by the value of another register (rhs) and set flags.
    /// The result is stored in acc.
    #[inline]
    fn div_reg_signed(acc: Register, rhs: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = processor.registers.get_reg(rhs);
        Self::div_val_signed(acc, val, processor);
    }

    /// Divide the value of a register (acc) by another value.
    /// The result is stored in the register.
    #[inline]
    fn div_val(acc: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        let a = processor.registers.get_reg(acc);
        processor.registers.set_reg(acc, a / val);
    }

    /// Divide the value of a register (acc) by another value and set flags.
    /// The result is stored in this register.
    #[inline]
    fn div_val_signed(acc: Register, val: W, processor: &mut Processor<STACK_SIZE, Self>) {
        let acc_val = processor.registers.get_reg(acc);
        let (res, overflow) = acc_val.overflowing_div(val);
        let carry = overflow; // this is the same as acc_val.carry_div(val)

        processor.registers.set_reg(acc, res);
        processor.registers.set_flag(Flag::V, overflow);
        processor.registers.set_flag(Flag::C, carry);

        Self::set_signed_zero_flags(res, processor);
    }

    /// Increment the value in a register by one.
    #[inline]
    fn inc(reg: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        processor.registers.inc(reg);
    }

    /// Increment the value in a register by one and set flags.
    #[inline]
    fn inc_signed(reg: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        Self::add_val_signed(reg, 1.into(), processor);
    }

    /// Decrement the value in a register by one.
    #[inline]
    fn dec(reg: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        processor.registers.dec(reg);
    }

    /// Decrement the value in a register by one and set flags.
    #[inline]
    fn dec_signed(reg: Register, processor: &mut Processor<STACK_SIZE, Self>) {
        Self::sub_val_signed(reg, 1.into(), processor);
    }

    /// Set program counter to value, effectively jumping to the instruction at this point in the program.
    #[inline]
    const fn jmp(to: W, processor: &mut Processor<STACK_SIZE, Self>) {
        processor.registers.set_reg(Register::PC, to);
    }

    /// Analog to `jmp` but only if zero flag is set.
    #[inline]
    const fn jmp_zero(to: W, processor: &mut Processor<STACK_SIZE, Self>) {
        if processor.registers.get_flag(Flag::Z) {
            processor.registers.set_reg(Register::PC, to);
        }
    }

    /// Analog to `jmp` but only if zero flag is not set.
    #[inline]
    const fn jmp_not_zero(to: W, processor: &mut Processor<STACK_SIZE, Self>) {
        if !processor.registers.get_flag(Flag::Z) {
            processor.registers.set_reg(Register::PC, to);
        }
    }

    /// Analog to `jmp` but only if carry flag is set.
    #[inline]
    const fn jmp_carry(to: W, processor: &mut Processor<STACK_SIZE, Self>) {
        if processor.registers.get_flag(Flag::C) {
            processor.registers.set_reg(Register::PC, to);
        }
    }

    /// Analog to `jmp` but only if carry flag is not set.
    #[inline]
    const fn jmp_not_carry(to: W, processor: &mut Processor<STACK_SIZE, Self>) {
        if !processor.registers.get_flag(Flag::C) {
            processor.registers.set_reg(Register::PC, to);
        }
    }

    /// Analog to `jmp` but only if signed flag is set.
    #[inline]
    const fn jmp_signed(to: W, processor: &mut Processor<STACK_SIZE, Self>) {
        if processor.registers.get_flag(Flag::S) {
            processor.registers.set_reg(Register::PC, to);
        }
    }

    /// Analog to `jmp` but only if signed flag is not set.
    #[inline]
    const fn jmp_not_signed(to: W, processor: &mut Processor<STACK_SIZE, Self>) {
        if !processor.registers.get_flag(Flag::S) {
            processor.registers.set_reg(Register::PC, to);
        }
    }

    /// Analog to `jmp` but only if zero flag and signed flag are not set.
    #[inline]
    const fn jmp_greater(to: W, processor: &mut Processor<STACK_SIZE, Self>) {
        if !processor.registers.get_flag(Flag::Z) && !processor.registers.get_flag(Flag::S) {
            processor.registers.set_reg(Register::PC, to);
        }
    }

    /// Analog to `jmp` but only if zero flag is not set and signed flag is set.
    #[inline]
    const fn jmp_less(to: W, processor: &mut Processor<STACK_SIZE, Self>) {
        if !processor.registers.get_flag(Flag::Z) && processor.registers.get_flag(Flag::S) {
            processor.registers.set_reg(Register::PC, to);
        }
    }

    /// Analog to `jmp` but only if zero flag is set or signed flag is not set.
    #[inline]
    const fn jmp_greater_or_eq(to: W, processor: &mut Processor<STACK_SIZE, Self>) {
        if processor.registers.get_flag(Flag::Z) || !processor.registers.get_flag(Flag::S) {
            processor.registers.set_reg(Register::PC, to);
        }
    }

    /// Analog to `jmp` but only if zero flag or signed flag is set.
    #[inline]
    const fn jmp_less_or_eq(to: W, processor: &mut Processor<STACK_SIZE, Self>) {
        if processor.registers.get_flag(Flag::Z) || processor.registers.get_flag(Flag::S) {
            processor.registers.set_reg(Register::PC, to);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use emulator_core::word::*;

    const STACK_SIZE: usize = 32;
    type IS = Instruction<STACK_SIZE, I8>;

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
