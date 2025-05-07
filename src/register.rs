use std::ops::AddAssign;

use thiserror::Error;

use crate::stack::Word;

/// Marker trait for register sizes.
pub trait RegisterSize: Copy + Default + Sized + AddAssign {}

impl RegisterSize for u8 {}
impl RegisterSize for u16 {}
impl RegisterSize for u32 {}
impl RegisterSize for u64 {}
impl RegisterSize for u128 {}
impl RegisterSize for usize {}

/// Macro to create the registers struct and a corresponding enum for the register names.
// TODO: User defined register count and sizes
// TODO: Consider changing registers to use a array for the registers and a enum for the names
macro_rules! def_registers {
    ($($register: ident),* $(,)?) => {
        /// Get count of registers
        #[allow(dead_code)]
        enum Idents { $($register,)* __Count__ }
        pub const COUNT: usize = Idents::__Count__ as usize;

        /// Registers struct
        #[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
        pub struct Registers<R: RegisterSize, W: Word> {
            pub general: [R; COUNT],
            pub pc: W,
            pub sp: W,
        }

        impl<R: RegisterSize, W: Word> Registers<R, W> {
            /// Create a new set of registers with all values initialized to the default value.
            pub fn new() -> Self {
                Registers {
                    general: [R::default(); COUNT],
                    pc: W::default(),
                    sp: W::default(),
                }
            }

            /// Get a register's value. The stack pointer and the program counter are not accessible directly.
            pub fn get(&self, register: Register) -> RegisterValue<R, W> {
                match register {
                    $(
                        // This should never panic as the general register array's length is calculated by the amount of general registers
                        Register::$register => RegisterValue::Other(self.general[Register::$register as usize]),
                    )*
                    Register::PC => RegisterValue::Word(self.pc),
                    Register::SP => RegisterValue::Word(self.sp),
                }
            }

            pub fn add(&mut self,register: Register, value: RegisterValue<R,W>)-> Result<(), RegisterError> {
                match register {
                    $(
                        // This should never panic as the general register array's length is calculated by the amount of general registers
                        Register::$register => match value {
                            RegisterValue::Word(_) => Err(RegisterError::InvalidGeneralRegisterValue),
                            RegisterValue::Other(other) => Ok(self.general[Register::$register as usize] += other),
                        },
                    )*
                    Register::PC => match value {
                        RegisterValue::Word(word) => Ok(self.pc += word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidProgramCounterValue),
                    },
                    Register::SP => match value {
                        RegisterValue::Word(word) => Ok(self.sp += word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidStackPointerValue),
                    }
                }
            }

            /// Set a register's value. The stack pointer and the program counter are not accessible directly.
            pub fn set(&mut self, register: Register, value: RegisterValue<R, W>) -> Result<(), RegisterError> {
                match register {
                    $(
                        // This should never panic as the general register array's length is calculated by the amount of general registers
                        Register::$register => match value {
                            RegisterValue::Word(_) => Err(RegisterError::InvalidGeneralRegisterValue),
                            RegisterValue::Other(other) => Ok(self.general[Register::$register as usize] = other),
                        },
                    )*
                    Register::PC => match value {
                        RegisterValue::Word(word) => Ok(self.pc = word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidProgramCounterValue),
                    },
                    Register::SP => match value {
                        RegisterValue::Word(word) => Ok(self.sp = word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidStackPointerValue),
                    }
                }
            }
        }

        #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
        pub enum RegisterValue<R, W> {
            Word(W),
            Other(R),
        }

        /// Enum of all register names
        #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
        pub enum Register {
            $(
                $register,
            )*
            PC,
            SP
        }
    };
}

def_registers!(
    R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14, R15
);

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum RegisterError {
    #[error("Invalid general register value")]
    InvalidGeneralRegisterValue,
    #[error("Invalid program counter value")]
    InvalidProgramCounterValue,
    #[error("Invalid stack pointer value")]
    InvalidStackPointerValue,
}
