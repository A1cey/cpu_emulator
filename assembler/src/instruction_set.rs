use core::cmp::Ordering;

use emulator_core::{
    instruction_set::InstructionSet,
    processor::Processor,
    register::{Flag, Register},
    word::Word,
};

/// Operand for the instruction set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operand<W: Word> {
    Register(Register),
    Value(W),
}

/// Operand for the instruction set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SignedOperation {
    Signed(),
    Unsigned(),
}

/// Jump condition for the instruction set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JumpCondition {
    /// No condition. \[JMP\]
    Unconditional,
    /// If zero flag is set. \[JZ\]
    Zero,
    /// If zero flag is not set. \[JNZ\]
    NotZero,
    /// If carry flag is set. \[JC\]
    Carry,
    /// If carry flag is not set. \[JNC\]
    NotCarry,
    /// If signed flag is set. \[JS\]
    Signed,
    /// If signed flag is not set. \[JNS\]
    NotSigned,
    /// If zero flag and signed flag are not set. \[JG\]
    Greater,
    /// If zero flag is not set and signed flag is set. \[JL\]
    Less,
    /// If zero flag is set or signed flag is not set. \[JGE\]
    GreaterOrEq,
    /// If zero flag or signed flag is set. \[JLE\]
    LessOrEq,
}

/// Default instruction set for the processor.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Instruction<const STACK_SIZE: usize, W: Word> {
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
}

impl<const STACK_SIZE: usize, W: Word> InstructionSet<STACK_SIZE> for Instruction<STACK_SIZE, W> {
    type Instruction = Self;
    type W = W;

    /// Execute an instruction on a processor.
    fn execute(instruction: &Self, processor: &mut Processor<STACK_SIZE, Self>) {
        match instruction {
            Self::Nop => (),
            Self::Mov { to, from } => Self::mov(*to, *from, processor),
            Self::Add { acc, rhs, signed } => Self::add(*acc, *rhs, *signed, processor),
            Self::Sub { acc, rhs, signed } => Self::sub(*acc, *rhs, *signed, processor),
            Self::Mul { acc, rhs, signed } => Self::mul(*acc, *rhs, *signed, processor),
            Self::Div { acc, rhs, signed } => Self::div(*acc, *rhs, *signed, processor),
            Self::Inc { reg, signed } => Self::inc(*reg, *signed, processor),
            Self::Dec { reg, signed } => Self::dec(*reg, *signed, processor),
            Self::Jump { to, condition } => Self::jmp(*to, *condition, processor),
        }
    }
}

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

impl<const STACK_SIZE: usize, W: Word> Instruction<STACK_SIZE, W> {
    pub(crate) const fn from_binary_instruction(
        instr: ASMBinaryInstruction,
        lhs: Register,
        rhs: Operand<W>,
    ) -> Self {
        match instr {
            ASMBinaryInstruction::Mov => Self::Mov { to: lhs, from: rhs },
            ASMBinaryInstruction::Add => Self::Add {
                acc: lhs,
                rhs,
                signed: false,
            },
            ASMBinaryInstruction::AddS => Self::Add {
                acc: lhs,
                rhs,
                signed: true,
            },
            ASMBinaryInstruction::Sub => Self::Sub {
                acc: lhs,
                rhs,
                signed: false,
            },
            ASMBinaryInstruction::SubS => Self::Sub {
                acc: lhs,
                rhs,
                signed: true,
            },
            ASMBinaryInstruction::Mul => Self::Mul {
                acc: lhs,
                rhs,
                signed: false,
            },
            ASMBinaryInstruction::MulS => Self::Mul {
                acc: lhs,
                rhs,
                signed: true,
            },
            ASMBinaryInstruction::Div => Self::Div {
                acc: lhs,
                rhs,
                signed: false,
            },
            ASMBinaryInstruction::DivS => Self::Div {
                acc: lhs,
                rhs,
                signed: true,
            },
        }
    }

    pub(crate) const fn from_unary_instruction(instr: ASMUnaryInstruction, reg: Register) -> Self {
        match instr {
            ASMUnaryInstruction::Inc => Self::Inc { reg, signed: false },
            ASMUnaryInstruction::IncS => Self::Inc { reg, signed: true },
            ASMUnaryInstruction::Dec => Self::Dec { reg, signed: false },
            ASMUnaryInstruction::DecS => Self::Dec { reg, signed: true },
        }
    }

    pub(crate) const fn from_jump_instruction(instr: ASMJumpInstruction, dest: W) -> Self {
        let condition = match instr {
            ASMJumpInstruction::Jmp => JumpCondition::Unconditional,
            ASMJumpInstruction::JZ => JumpCondition::Zero,
            ASMJumpInstruction::JNZ => JumpCondition::NotZero,
            ASMJumpInstruction::JC => JumpCondition::Carry,
            ASMJumpInstruction::JNC => JumpCondition::NotCarry,
            ASMJumpInstruction::JS => JumpCondition::Signed,
            ASMJumpInstruction::JNS => JumpCondition::NotSigned,
            ASMJumpInstruction::JG => JumpCondition::Greater,
            ASMJumpInstruction::JL => JumpCondition::Less,
            ASMJumpInstruction::JGE => JumpCondition::GreaterOrEq,
            ASMJumpInstruction::JLE => JumpCondition::LessOrEq,
        };

        Self::Jump {
            to: dest,
            condition,
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

    /// Copy a value from an operand to a register.
    #[inline]
    const fn mov(to: Register, from: Operand<W>, processor: &mut Processor<STACK_SIZE, Self>) {
        let val = match from {
            Operand::Register(reg) => processor.registers.get_reg(reg),
            Operand::Value(val) => val,
        };

        processor.registers.set_reg(to, val);
    }

    /// Add the value of an operand (rhs) to a register (acc).
    #[inline]
    fn add(
        acc: Register,
        rhs: Operand<W>,
        signed: bool,
        processor: &mut Processor<STACK_SIZE, Self>,
    ) {
        let val = match rhs {
            Operand::Register(reg) => processor.registers.get_reg(reg),
            Operand::Value(val) => val,
        };

        let a = processor.registers.get_reg(acc);

        if signed {
            let (res, overflow) = a.overflowing_add(val);
            let carry = a.carry_add(val);

            processor.registers.set_reg(acc, res);
            processor.registers.set_flag(Flag::V, overflow);
            processor.registers.set_flag(Flag::C, carry);

            Self::set_signed_zero_flags(res, processor);
        } else {
            processor.registers.set_reg(acc, a + val);
        }
    }

    /// Subtract the value of an operand (rhs) from a register (acc).
    #[inline]
    fn sub(
        acc: Register,
        rhs: Operand<W>,
        signed: bool,
        processor: &mut Processor<STACK_SIZE, Self>,
    ) {
        let val = match rhs {
            Operand::Register(reg) => processor.registers.get_reg(reg),
            Operand::Value(val) => val,
        };

        let a = processor.registers.get_reg(acc);

        if signed {
            let (res, overflow) = a.overflowing_sub(val);
            let carry = a.carry_sub(val);

            processor.registers.set_reg(acc, res);
            processor.registers.set_flag(Flag::V, overflow);
            processor.registers.set_flag(Flag::C, carry);

            Self::set_signed_zero_flags(res, processor);
        } else {
            processor.registers.set_reg(acc, a - val);
        }
    }

    /// Multiply the value of an operand (acc) with the value of a register (rhs).
    /// The result is stored in acc.
    #[inline]
    fn mul(
        acc: Register,
        rhs: Operand<W>,
        signed: bool,
        processor: &mut Processor<STACK_SIZE, Self>,
    ) {
        let val = match rhs {
            Operand::Register(reg) => processor.registers.get_reg(reg),
            Operand::Value(val) => val,
        };

        let a = processor.registers.get_reg(acc);

        if signed {
            let (res, overflow) = a.overflowing_mul(val);
            let carry = a.carry_mul(val);

            processor.registers.set_reg(acc, res);
            processor.registers.set_flag(Flag::V, overflow);
            processor.registers.set_flag(Flag::C, carry);

            Self::set_signed_zero_flags(res, processor);
        } else {
            processor.registers.set_reg(acc, a * val);
        }
    }

    /// Divide the value of an operand (acc) by the value of a register (rhs).
    /// The result is stored in acc.
    #[inline]
    fn div(
        acc: Register,
        rhs: Operand<W>,
        signed: bool,
        processor: &mut Processor<STACK_SIZE, Self>,
    ) {
        let val = match rhs {
            Operand::Register(reg) => processor.registers.get_reg(reg),
            Operand::Value(val) => val,
        };

        let a = processor.registers.get_reg(acc);

        if signed {
            let (res, overflow) = a.overflowing_div(val);
            let carry = overflow; // this is the same as a.carry_div(val)

            processor.registers.set_reg(acc, res);
            processor.registers.set_flag(Flag::V, overflow);
            processor.registers.set_flag(Flag::C, carry);

            Self::set_signed_zero_flags(res, processor);
        } else {
            processor.registers.set_reg(acc, a / val);
        }
    }

    /// Increment the value in a register by one.
    #[inline]
    fn inc(reg: Register, signed: bool, processor: &mut Processor<STACK_SIZE, Self>) {
        if signed {
            Self::add(reg, Operand::Value(1.into()), true, processor);
        } else {
            processor.registers.inc(reg);
        }
    }

    /// Decrement the value in a register by one.
    #[inline]
    fn dec(reg: Register, signed: bool, processor: &mut Processor<STACK_SIZE, Self>) {
        if signed {
            Self::sub(reg, Operand::Value(1.into()), true, processor);
        } else {
            processor.registers.dec(reg);
        }
    }

    /// Set program counter to value, effectively jumping to the instruction at this point in the program.
    #[inline]
    const fn jmp(to: W, condition: JumpCondition, processor: &mut Processor<STACK_SIZE, Self>) {
        if match condition {
            JumpCondition::Unconditional => true,
            JumpCondition::Zero => processor.registers.get_flag(Flag::Z),
            JumpCondition::NotZero => !processor.registers.get_flag(Flag::Z),
            JumpCondition::Carry => processor.registers.get_flag(Flag::C),
            JumpCondition::NotCarry => !processor.registers.get_flag(Flag::C),
            JumpCondition::Signed => processor.registers.get_flag(Flag::S),
            JumpCondition::NotSigned => !processor.registers.get_flag(Flag::S),
            JumpCondition::Greater => {
                !processor.registers.get_flag(Flag::Z) && !processor.registers.get_flag(Flag::S)
            }
            JumpCondition::Less => {
                !processor.registers.get_flag(Flag::Z) && processor.registers.get_flag(Flag::S)
            }
            JumpCondition::GreaterOrEq => {
                processor.registers.get_flag(Flag::Z) || !processor.registers.get_flag(Flag::S)
            }
            JumpCondition::LessOrEq => {
                processor.registers.get_flag(Flag::Z) || processor.registers.get_flag(Flag::S)
            }
        } {
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
            &Instruction::Mov {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        let _ = IS::execute(
            &Instruction::Mov {
                to: Register::R0,
                from: Operand::Value(10.into()),
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 10.into());
    }

    #[test]
    fn test_inc() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        let _ = IS::execute(
            &Instruction::Inc {
                reg: Register::R0,
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 11.into());
    }

    #[test]
    fn test_inc_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MAX.into());
        let _ = IS::execute(
            &Instruction::Inc {
                reg: Register::R0,
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), i8::MIN.into());
    }

    #[test]
    fn test_dec() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        let _ = IS::execute(
            &Instruction::Dec {
                reg: Register::R0,
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 9.into());
    }

    #[test]
    fn test_dec_underflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MIN.into());
        let _ = IS::execute(
            &Instruction::Dec {
                reg: Register::R0,
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), i8::MAX.into());
    }

    #[test]
    fn test_add_reg() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        processor.registers.set_reg(Register::R1, 10.into());
        let _ = IS::execute(
            &Instruction::Add {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MAX.into());
        processor.registers.set_reg(Register::R1, 1.into());
        let _ = IS::execute(
            &Instruction::Add {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        let _ = IS::execute(
            &Instruction::Add {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MAX.into());
        let _ = IS::execute(
            &Instruction::Add {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        processor.registers.set_reg(Register::R1, 10.into());
        let _ = IS::execute(
            &Instruction::Sub {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MIN.into());
        processor.registers.set_reg(Register::R1, 1.into());
        let _ = IS::execute(
            &Instruction::Sub {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        let _ = IS::execute(
            &Instruction::Sub {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, (-128).into());
        let _ = IS::execute(
            &Instruction::Sub {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        processor.registers.set_reg(Register::R1, 10.into());
        let _ = IS::execute(
            &Instruction::Mul {
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
            &Instruction::Mul {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 80.into());
        processor.registers.set_reg(Register::R1, 2.into());
        let _ = IS::execute(
            &Instruction::Mul {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, (-80).into());
        processor.registers.set_reg(Register::R1, 2.into());
        let _ = IS::execute(
            &Instruction::Mul {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 5.into());
        let _ = IS::execute(
            &Instruction::Mul {
                acc: Register::R0,
                rhs: Operand::Value(10.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 50.into());

        processor.registers.set_reg(Register::R0, (-5).into());
        let _ = IS::execute(
            &Instruction::Mul {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 80.into());
        let _ = IS::execute(
            &Instruction::Mul {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, (-80).into());
        let _ = IS::execute(
            &Instruction::Mul {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        processor.registers.set_reg(Register::R1, 5.into());
        let _ = IS::execute(
            &Instruction::Div {
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
            &Instruction::Div {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 3.into());
        processor.registers.set_reg(Register::R1, 2.into());
        let _ = IS::execute(
            &Instruction::Div {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MIN.into());
        processor.registers.set_reg(Register::R1, (-1).into());
        let _ = IS::execute(
            &Instruction::Div {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 10.into());
        let _ = IS::execute(
            &Instruction::Div {
                acc: Register::R0,
                rhs: Operand::Value(5.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 2.into());

        processor.registers.set_reg(Register::R0, (-10).into());
        let _ = IS::execute(
            &Instruction::Div {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, 3.into());
        let _ = IS::execute(
            &Instruction::Div {
                acc: Register::R0,
                rhs: Operand::Value(4.into()),
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::R0), 0.into());

        processor.registers.set_reg(Register::R0, 3.into());
        let _ = IS::execute(
            &Instruction::Div {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        processor.registers.set_reg(Register::R0, i8::MIN.into());
        let _ = IS::execute(
            &Instruction::Div {
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
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        assert_eq!(processor.registers.get_reg(Register::PC), 0.into());
        let _ = IS::execute(
            &Instruction::Jump {
                to: 2.into(),
                condition: JumpCondition::Unconditional,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::PC), 2.into());
    }

    #[test]
    fn test_jmp_overflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        assert_eq!(processor.registers.get_reg(Register::PC), 0.into());
        let _ = IS::execute(
            &Instruction::Jump {
                to: i8::MAX.into(),
                condition: JumpCondition::Unconditional,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::PC), i8::MAX.into());
        let _ = IS::execute(
            &Instruction::Inc {
                reg: Register::PC,
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::PC), i8::MIN.into());
    }

    #[test]
    fn test_jmp_underflow() {
        let mut processor = Processor::<STACK_SIZE, IS>::new();
        assert_eq!(processor.registers.get_reg(Register::PC), 0.into());
        let _ = IS::execute(
            &Instruction::Jump {
                to: i8::MIN.into(),
                condition: JumpCondition::Unconditional,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::PC), i8::MIN.into());
        let _ = IS::execute(
            &Instruction::Dec {
                reg: Register::PC,
                signed: false,
            },
            &mut processor,
        );
        assert_eq!(processor.registers.get_reg(Register::PC), i8::MAX.into());
    }
}
