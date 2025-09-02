pub(crate) mod asm_instruction;
pub mod jump_condition;
pub mod operand;

use core::cmp::Ordering;
use std::ops::Deref;

use emulator_core::{
    instruction_set::InstructionSet,
    processor::Processor,
    register::{Flag, Register},
    word::Word,
};

use crate::instruction::{
    asm_instruction::{ASMBinaryInstruction, ASMJumpInstruction, ASMUnaryInstruction},
    jump_condition::JumpCondition,
    operand::Operand,
};

/// Default instruction set for the processor.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum Instruction<W: Word> {
    /// No operation. \[NOP\]
    Nop,
    /// Copy a value from the operand to the register. \[MOV\]
    Mov { to: Register, from: Operand<W> },
    /// Add the value of the operand (rhs) to the register (acc).
    /// The result is stored in acc. \[ADD\]
    Add {
        acc: Register,
        rhs: Operand<W>,
        signed: bool,
    },
    /// Subtract the value of the operand (rhs) from the register (acc).
    /// The result is stored in acc. \[SUB\]
    Sub {
        acc: Register,
        rhs: Operand<W>,
        signed: bool,
    },
    /// Multiply the value of the operand (rhs) with the value of the register (acc).
    /// The result is stored in acc. \[MUL\]
    Mul {
        acc: Register,
        rhs: Operand<W>,
        signed: bool,
    },
    /// Divide the value of the register (acc) by the value of the operand (rhs).
    /// The result is stored in acc. \[DIV\]
    Div {
        acc: Register,
        rhs: Operand<W>,
        signed: bool,
    },
    /// Increment the value in a register by one. \[INC\]
    Inc { reg: Register, signed: bool },
    /// Decrement the value in a register by one. \[DEC\]
    Dec { reg: Register, signed: bool },
    /// Set program pointer to value, effectively jumping to the instruction at this point in the program.
    /// The condition is checked before jumping and the jump is performed if the condition is met.
    /// See the assembly instruction at `JumpCondition`.
    Jump { to: W, condition: JumpCondition },
    // Cmp
    // Push
    // Pop
    // Call
    // Ret
    // Xor
    // And
    // Or
    // Not
    // Shl
    // Shr
    // Rol
    // Ror
    // Load
    // Store
}

impl<W: Word> InstructionSet for Instruction<W> {
    type Instruction = Self;
    type W = W;

    /// Execute an instruction on a processor.
    fn execute<const STACK_SIZE: usize, P: Deref<Target = [Self::Instruction]>>(
        instruction: Self,
        processor: &mut Processor<STACK_SIZE, Self, P>,
    ) {
        use Instruction::{Add, Dec, Div, Inc, Jump, Mov, Mul, Nop, Sub};

        match instruction {
            Nop => (),
            Mov { to, from } => Self::mov(to, from, processor),
            Add { acc, rhs, signed } => Self::add(acc, rhs, signed, processor),
            Sub { acc, rhs, signed } => Self::sub(acc, rhs, signed, processor),
            Mul { acc, rhs, signed } => Self::mul(acc, rhs, signed, processor),
            Div { acc, rhs, signed } => Self::div(acc, rhs, signed, processor),
            Inc { reg, signed } => Self::inc(reg, signed, processor),
            Dec { reg, signed } => Self::dec(reg, signed, processor),
            Jump { to, condition } => {
                if condition.check(processor) {
                    processor.registers.set_reg(Register::PC, to);
                }
            }
        }
    }
}

impl<W: Word> Instruction<W> {
    // skips forrmatting the match
    #[rustfmt::skip]
    pub(crate) const fn from_binary_instruction(
        instr: ASMBinaryInstruction,
        lhs: Register,
        rhs: Operand<W>
    ) -> Self {
        use ASMBinaryInstruction::{Mov, Add, AddS, Sub, SubS, Mul, MulS, Div, DivS};
        match instr {
            Mov => Self::Mov { to: lhs, from: rhs },
            Add => Self::Add { acc: lhs, rhs, signed: false },
            AddS => Self::Add { acc: lhs, rhs, signed: true },
            Sub => Self::Sub { acc: lhs, rhs, signed: false },
            SubS => Self::Sub { acc: lhs, rhs, signed: true },
            Mul => Self::Mul { acc: lhs, rhs, signed: false },
            MulS => Self::Mul { acc: lhs, rhs, signed: true },
            Div => Self::Div { acc: lhs, rhs, signed: false },
            DivS => Self::Div { acc: lhs, rhs, signed: true },
        }
    }

    pub(crate) const fn from_unary_instruction(instr: ASMUnaryInstruction, reg: Register) -> Self {
        use ASMUnaryInstruction::{Dec, DecS, Inc, IncS};
        match instr {
            Inc => Self::Inc { reg, signed: false },
            IncS => Self::Inc { reg, signed: true },
            Dec => Self::Dec { reg, signed: false },
            DecS => Self::Dec { reg, signed: true },
        }
    }

    pub(crate) const fn from_jump_instruction(instr: ASMJumpInstruction, dest: W) -> Self {
        use ASMJumpInstruction::{Jc, Jg, Jge, Jl, Jle, Jmp, Jnc, Jns, Jnz, Js, Jz};
        let condition = match instr {
            Jmp => JumpCondition::Unconditional,
            Jz => JumpCondition::Zero,
            Jnz => JumpCondition::NotZero,
            Jc => JumpCondition::Carry,
            Jnc => JumpCondition::NotCarry,
            Js => JumpCondition::Signed,
            Jns => JumpCondition::NotSigned,
            Jg => JumpCondition::Greater,
            Jl => JumpCondition::Less,
            Jge => JumpCondition::GreaterOrEq,
            Jle => JumpCondition::LessOrEq,
        };

        Self::Jump { to: dest, condition }
    }

    /// Copy a value from an operand to a register.
    #[inline]
    const fn mov<const STACK_SIZE: usize, P: Deref<Target = [Self]>>(
        to: Register,
        from: Operand<W>,
        processor: &mut Processor<STACK_SIZE, Self, P>,
    ) {
        let val = match from {
            Operand::Register(reg) => processor.registers.get_reg(reg),
            Operand::Value(val) => val,
        };

        processor.registers.set_reg(to, val);
    }

    /// Add the value of an operand (rhs) to a register (acc).
    #[inline]
    fn add<const STACK_SIZE: usize, P: Deref<Target = [Self]>>(
        acc: Register,
        rhs: Operand<W>,
        signed: bool,
        processor: &mut Processor<STACK_SIZE, Self, P>,
    ) {
        let a = processor.registers.get_reg(acc);
        let b = rhs.resolve(processor);

        if signed {
            let (result, overflow) = a.overflowing_add(b);
            let carry = a.check_carry_add(b);

            processor.registers.set_reg(acc, result);
            processor.registers.set_flag(Flag::V, overflow);
            processor.registers.set_flag(Flag::C, carry);

            Self::set_signed_zero_flags(result, processor);
        } else {
            processor.registers.set_reg(acc, a + b);
        }
    }

    /// Subtract the value of an operand (rhs) from a register (acc).
    #[inline]
    fn sub<const STACK_SIZE: usize, P: Deref<Target = [Self]>>(
        acc: Register,
        rhs: Operand<W>,
        signed: bool,
        processor: &mut Processor<STACK_SIZE, Self, P>,
    ) {
        let a = processor.registers.get_reg(acc);
        let b = rhs.resolve(processor);

        if signed {
            let (result, overflow) = a.overflowing_sub(b);
            let carry = a.check_carry_sub(b);

            processor.registers.set_reg(acc, result);
            processor.registers.set_flag(Flag::V, overflow);
            processor.registers.set_flag(Flag::C, carry);

            Self::set_signed_zero_flags(result, processor);
        } else {
            processor.registers.set_reg(acc, a - b);
        }
    }

    /// Multiply the value of an operand (acc) with the value of a register (rhs).
    /// The result is stored in acc.
    #[inline]
    fn mul<const STACK_SIZE: usize, P: Deref<Target = [Self]>>(
        acc: Register,
        rhs: Operand<W>,
        signed: bool,
        processor: &mut Processor<STACK_SIZE, Self, P>,
    ) {
        let a = processor.registers.get_reg(acc);
        let b = rhs.resolve(processor);

        if signed {
            let (result, overflow) = a.overflowing_mul(b);
            let carry = a.check_carry_mul(b);

            processor.registers.set_reg(acc, result);
            processor.registers.set_flag(Flag::V, overflow);
            processor.registers.set_flag(Flag::C, carry);

            Self::set_signed_zero_flags(result, processor);
        } else {
            processor.registers.set_reg(acc, a * b);
        }
    }

    /// Divide the value of an operand (acc) by the value of a register (rhs).
    /// The result is stored in acc.
    #[inline]
    fn div<const STACK_SIZE: usize, P: Deref<Target = [Self]>>(
        acc: Register,
        rhs: Operand<W>,
        signed: bool,
        processor: &mut Processor<STACK_SIZE, Self, P>,
    ) {
        let a = processor.registers.get_reg(acc);
        let b = rhs.resolve(processor);

        if signed {
            let (result, overflow) = a.overflowing_div(b);
            let carry = overflow; // this is the same as a.carry_div(b)

            processor.registers.set_reg(acc, result);
            processor.registers.set_flag(Flag::V, overflow);
            processor.registers.set_flag(Flag::C, carry);

            Self::set_signed_zero_flags(result, processor);
        } else {
            processor.registers.set_reg(acc, a / b);
        }
    }

    /// Increment the value in a register by one.
    #[inline]
    fn inc<const STACK_SIZE: usize, P: Deref<Target = [Self]>>(
        reg: Register,
        signed: bool,
        processor: &mut Processor<STACK_SIZE, Self, P>,
    ) {
        if signed {
            Self::add(reg, Operand::Value(1.into()), true, processor);
        } else {
            processor.registers.inc(reg);
        }
    }

    /// Decrement the value in a register by one.
    #[inline]
    fn dec<const STACK_SIZE: usize, P: Deref<Target = [Self]>>(
        reg: Register,
        signed: bool,
        processor: &mut Processor<STACK_SIZE, Self, P>,
    ) {
        if signed {
            Self::sub(reg, Operand::Value(1.into()), true, processor);
        } else {
            processor.registers.dec(reg);
        }
    }

    /// Sets the signed and zero flags.
    #[inline]
    fn set_signed_zero_flags<const STACK_SIZE: usize, P: Deref<Target = [Self]>>(
        val: W,
        processor: &mut Processor<STACK_SIZE, Self, P>,
    ) {
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
}

#[cfg(test)]
mod test {

    use super::*;
    use emulator_core::word::*;

    const STACK_SIZE: usize = 32;
    type IS = Instruction<I8>;
    type P = Vec<IS>;

    #[test]
    fn test_move_reg() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        let _ = IS::execute(
            Instruction::Mov {
                from: Operand::Register(Register::R0),
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
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        let _ = IS::execute(
            Instruction::Mov {
                to: Register::R0,
                from: Operand::Value(10.into()),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 10.into());
    }

    #[test]
    fn test_inc() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        let _ = IS::execute(
            Instruction::Inc {
                reg: Register::R0,
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 11.into());
    }

    #[test]
    fn test_inc_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, i8::MAX.into());
        let _ = IS::execute(
            Instruction::Inc {
                reg: Register::R0,
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), i8::MIN.into());
    }

    #[test]
    fn test_dec() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        let _ = IS::execute(
            Instruction::Dec {
                reg: Register::R0,
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 9.into());
    }

    #[test]
    fn test_dec_underflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, i8::MIN.into());
        let _ = IS::execute(
            Instruction::Dec {
                reg: Register::R0,
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), i8::MAX.into());
    }

    #[test]
    fn test_add_reg() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        processor.registers.set_reg(Register::R1, 10.into());
        let _ = IS::execute(
            Instruction::Add {
                acc: Register::R0,
                rhs: Operand::Register(Register::R1),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 15.into());
    }

    #[test]
    fn test_add_reg_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, i8::MAX.into());
        processor.registers.set_reg(Register::R1, 1.into());
        let _ = IS::execute(
            Instruction::Add {
                acc: Register::R0,
                rhs: Operand::Register(Register::R1),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), i8::MIN.into());
    }

    #[test]
    fn test_add_val() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        let _ = IS::execute(
            Instruction::Add {
                acc: Register::R0,
                rhs: Operand::Value(10.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 15.into());
    }

    #[test]
    fn test_add_val_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, i8::MAX.into());
        let _ = IS::execute(
            Instruction::Add {
                acc: Register::R0,
                rhs: Operand::Value(1.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), i8::MIN.into());
    }

    #[test]
    fn test_sub_reg() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        processor.registers.set_reg(Register::R1, 10.into());
        let _ = IS::execute(
            Instruction::Sub {
                acc: Register::R0,
                rhs: Operand::Register(Register::R1),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-5).into());
    }

    #[test]
    fn test_sub_reg_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, i8::MIN.into());
        processor.registers.set_reg(Register::R1, 1.into());
        let _ = IS::execute(
            Instruction::Sub {
                acc: Register::R0,
                rhs: Operand::Register(Register::R1),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), i8::MAX.into());
    }

    #[test]
    fn test_sub_val() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        let _ = IS::execute(
            Instruction::Sub {
                acc: Register::R0,
                rhs: Operand::Value(10.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-5).into());
    }

    #[test]
    fn test_sub_val_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, (-128).into());
        let _ = IS::execute(
            Instruction::Sub {
                acc: Register::R0,
                rhs: Operand::Value(1.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 127.into());
    }

    #[test]
    fn test_mul_reg() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        processor.registers.set_reg(Register::R1, 10.into());
        let _ = IS::execute(
            Instruction::Mul {
                acc: Register::R0,
                rhs: Operand::Register(Register::R1),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 50.into());

        processor.registers.set_reg(Register::R0, (-5).into());
        processor.registers.set_reg(Register::R1, 10.into());
        let _ = IS::execute(
            Instruction::Mul {
                acc: Register::R0,
                rhs: Operand::Register(Register::R1),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-50).into());
    }

    #[test]
    fn test_mul_reg_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 80.into());
        processor.registers.set_reg(Register::R1, 2.into());
        let _ = IS::execute(
            Instruction::Mul {
                acc: Register::R0,
                rhs: Operand::Register(Register::R1),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-96).into());
    }

    #[test]
    fn test_mul_reg_underflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, (-80).into());
        processor.registers.set_reg(Register::R1, 2.into());
        let _ = IS::execute(
            Instruction::Mul {
                acc: Register::R0,
                rhs: Operand::Register(Register::R1),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 96.into());
    }

    #[test]
    fn test_mul_val() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        let _ = IS::execute(
            Instruction::Mul {
                acc: Register::R0,
                rhs: Operand::Value(10.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 50.into());

        processor.registers.set_reg(Register::R0, (-5).into());
        let _ = IS::execute(
            Instruction::Mul {
                acc: Register::R0,
                rhs: Operand::Value(10.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-50).into());
    }

    #[test]
    fn test_mul_val_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 80.into());
        let _ = IS::execute(
            Instruction::Mul {
                acc: Register::R0,
                rhs: Operand::Value(2.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-96).into());
    }

    #[test]
    fn test_mul_val_underflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, (-80).into());
        let _ = IS::execute(
            Instruction::Mul {
                acc: Register::R0,
                rhs: Operand::Value(2.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 96.into());
    }

    #[test]
    fn test_div_reg() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        processor.registers.set_reg(Register::R1, 5.into());
        let _ = IS::execute(
            Instruction::Div {
                acc: Register::R0,
                rhs: Operand::Register(Register::R1),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 2.into());

        processor.registers.set_reg(Register::R0, (-10).into());
        processor.registers.set_reg(Register::R1, 5.into());
        let _ = IS::execute(
            Instruction::Div {
                acc: Register::R0,
                rhs: Operand::Register(Register::R1),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-2).into());
    }

    #[test]
    fn test_div_reg_truncate() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 3.into());
        processor.registers.set_reg(Register::R1, 2.into());
        let _ = IS::execute(
            Instruction::Div {
                acc: Register::R0,
                rhs: Operand::Register(Register::R1),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 1.into());
    }

    #[test]
    fn test_div_reg_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, i8::MIN.into());
        processor.registers.set_reg(Register::R1, (-1).into());
        let _ = IS::execute(
            Instruction::Div {
                acc: Register::R0,
                rhs: Operand::Register(Register::R1),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (i8::MIN).into());
    }

    #[test]
    fn test_div_val() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        let _ = IS::execute(
            Instruction::Div {
                acc: Register::R0,
                rhs: Operand::Value(5.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 2.into());

        processor.registers.set_reg(Register::R0, (-10).into());
        let _ = IS::execute(
            Instruction::Div {
                acc: Register::R0,
                rhs: Operand::Value(5.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (-2).into());
    }

    #[test]
    fn test_div_val_truncate() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, 3.into());
        let _ = IS::execute(
            Instruction::Div {
                acc: Register::R0,
                rhs: Operand::Value(4.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 0.into());

        processor.registers.set_reg(Register::R0, 3.into());
        let _ = IS::execute(
            Instruction::Div {
                acc: Register::R0,
                rhs: Operand::Value(2.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 1.into());
    }

    #[test]
    fn test_div_val_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        processor.registers.set_reg(Register::R0, i8::MIN.into());
        let _ = IS::execute(
            Instruction::Div {
                acc: Register::R0,
                rhs: Operand::Value((-1).into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), (i8::MIN).into());
    }

    #[test]
    fn test_jmp() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        assert_eq!(processor.registers.get_reg(Register::PC), 0.into());
        let _ = IS::execute(
            Instruction::Jump {
                to: 2.into(),
                condition: JumpCondition::Unconditional,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::PC), 2.into());
    }

    #[test]
    fn test_jmp_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        assert_eq!(processor.registers.get_reg(Register::PC), 0.into());
        let _ = IS::execute(
            Instruction::Jump {
                to: i8::MAX.into(),
                condition: JumpCondition::Unconditional,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::PC), i8::MAX.into());
        let _ = IS::execute(
            Instruction::Inc {
                reg: Register::PC,
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::PC), i8::MIN.into());
    }

    #[test]
    fn test_jmp_underflow() {
        let mut processor = Processor::<STACK_SIZE, IS, P>::new();
        assert_eq!(processor.registers.get_reg(Register::PC), 0.into());
        let _ = IS::execute(
            Instruction::Jump {
                to: i8::MIN.into(),
                condition: JumpCondition::Unconditional,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::PC), i8::MIN.into());
        let _ = IS::execute(
            Instruction::Dec {
                reg: Register::PC,
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::PC), i8::MAX.into());
    }
}
