use core::fmt::Debug;
use core::str::FromStr;
use thiserror::Error;

use crate::word::Word;

const GENERAL_REGISTER_COUNT: usize = 16;

/// Registers struct.
/// The register sizes correspond to the stack word size.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Registers<W> {
    /// general purpose registers
    general: [W; GENERAL_REGISTER_COUNT],
    /// program counter
    pub pc: W,
    /// stack pointer
    pub sp: W,
    /// flags: carry flag (C), signed flag (S), overflow flag (V), zero condition flag (Z)
    flags: [bool; 4],
}

impl<W: Word> Default for Registers<W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<W: Word> Registers<W> {
    /// Create a new set of registers with all values initialized to the default value.
    #[must_use]
    pub fn new() -> Self {
        Self {
            general: [W::default(); GENERAL_REGISTER_COUNT],
            pc: W::default(),
            sp: W::default(),
            flags: [false; 4],
        }
    }

    /// Get the value of a register.
    #[inline]
    pub const fn get_reg(&self, reg: Register) -> W {
        match reg {
            Register::PC => self.pc,
            Register::SP => self.sp,
            _ => self.general[reg as usize],
        }
    }

    /// Set the value of a register.
    #[inline]
    pub const fn set_reg(&mut self, reg: Register, val: W) {
        match reg {
            Register::PC => self.pc = val,
            Register::SP => self.sp = val,
            _ => self.general[reg as usize] = val,
        }
    }

    /// Get the value of a flag.
    #[inline]
    pub const fn get_flag(&self, f: Flag) -> bool {
        match f {
            Flag::C => self.flags[0],
            Flag::S => self.flags[1],
            Flag::V => self.flags[2],
            Flag::Z => self.flags[3],
        }
    }

    /// Set the value of a flag.
    #[inline]
    pub const fn set_flag(&mut self, f: Flag, val: bool) {
        self.flags[f as usize] = val;
    }

    /// Increment the value in a register by one.
    #[inline]
    pub fn inc(&mut self, reg: Register) {
        match reg {
            Register::PC => self.pc += 1.into(),
            Register::SP => self.sp += 1.into(),
            _ => self.general[reg as usize] += 1.into(),
        }
    }

    /// Decrement the value in a register by one.
    #[inline]
    pub fn dec(&mut self, reg: Register) {
        match reg {
            Register::PC => self.pc -= 1.into(),
            Register::SP => self.sp -= 1.into(),
            _ => self.general[reg as usize] -= 1.into(),
        }
    }

    fn fmt_general_registers(&self) -> String {
        let s = self
            .general
            .iter()
            .map(ToString::to_string)
            .fold(String::new(), |acc, reg| acc + "," + &reg);
        format!("[{s}]")
    }

    fn fmt_flags(&self) -> String {
        let mut s = String::new();

        s.push_str(format!("C: {}, ", self.flags[0]).as_str());
        s.push_str(format!("S: {}, ", self.flags[1]).as_str());
        s.push_str(format!("V: {}", self.flags[2]).as_str());
        s.push_str(format!("Z: {}", self.flags[3]).as_str());
        format!("[{s}]")
    }
}

impl<W: Word> ::core::fmt::Display for Registers<W> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error> {
        write!(
            f,
            "general:\t{}\npc:\t\t{}\nsp:\t\t{}\nflags:\t\t{}",
            self.fmt_general_registers(),
            self.pc,
            self.sp,
            self.fmt_flags()
        )
    }
}

/// Enum of all register names
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    PC,
    SP,
}

impl FromStr for Register {
    type Err = RegisterError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "R0" | "r0" => Ok(Self::R0),
            "R1" | "r1" => Ok(Self::R1),
            "R2" | "r2" => Ok(Self::R2),
            "R3" | "r3" => Ok(Self::R3),
            "R4" | "r4" => Ok(Self::R4),
            "R5" | "r5" => Ok(Self::R5),
            "R6" | "r6" => Ok(Self::R6),
            "R7" | "r7" => Ok(Self::R7),
            "R8" | "r8" => Ok(Self::R8),
            "R9" | "r9" => Ok(Self::R9),
            "R10" | "r10" => Ok(Self::R10),
            "R11" | "r11" => Ok(Self::R11),
            "R12" | "r12" => Ok(Self::R12),
            "R13" | "r13" => Ok(Self::R13),
            "R14" | "r14" => Ok(Self::R14),
            "R15" | "r15" => Ok(Self::R15),
            "PC" | "pc" => Ok(Self::PC),
            "SP" | "sp" => Ok(Self::SP),
            _ => Err(RegisterError::ConversionFailed {
                input: value.to_string(),
            }),
        }
    }
}

/// Flag registers
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Flag {
    /// Carry flag. Normally set when an addition results in a carry or a subtraction results in a borrow.
    C,
    /// Signed flag. Normally set when the last arithmetic computation resulted in a negative value.
    S,
    /// Overflow flag. Normally set when the last arithmetic computation resulted in an overflow.
    V,
    /// Zero condition flag. Normally set when the last arithmetic, logical or bitwise computation resulted in zero.
    Z,
}

#[derive(Debug, Error, Clone, PartialEq, Eq, Hash)]
pub enum RegisterError {
    #[error("Failed to convert {input} into a register.")]
    ConversionFailed { input: String },
}
