use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use thiserror::Error;

use crate::stack::Word;

/// Marker trait for register sizes.
pub trait RegisterSize:
    Copy + Default + From<u8> + AddAssign + SubAssign + MulAssign + DivAssign
{
}

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

            /// Get the value of a register.
            #[inline]
            pub fn get(&self, reg: Register) -> RegisterValue<R, W> {
                match reg {
                    $(
                        // This will never panic as the general register array's length is calculated by the amount of general registers
                        Register::$register => RegisterValue::Other(self.general[Register::$register as usize]),
                    )*
                    Register::PC => RegisterValue::Word(self.pc),
                    Register::SP => RegisterValue::Word(self.sp),
                }
            }

            /// Set the value of a register.
            ///
            /// # Errors
            ///
            /// This function will return an error if the type of the value does not match the type of the register.
            #[inline]
            pub fn set(&mut self, reg: Register, val: RegisterValue<R, W>) -> Result<(), RegisterError> {
                match reg {
                    $(
                        Register::$register => match val {
                            RegisterValue::Word(_) => Err(RegisterError::InvalidGeneralRegisterValue),
                            // This will never panic as the general register array's length is calculated by the amount of general registers
                            RegisterValue::Other(other) => Ok(self.general[Register::$register as usize] = other),
                        },
                    )*
                    Register::PC => match val {
                        RegisterValue::Word(word) => Ok(self.pc = word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidProgramCounterValue),
                    },
                    Register::SP => match val {
                        RegisterValue::Word(word) => Ok(self.sp = word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidStackPointerValue),
                    }
                }
            }

            /// Add a value to a register.
            ///
            /// # Errors
            ///
            /// This function will return an error if the type of the value does not match the type of the register.
            #[inline]
            pub fn add(&mut self, reg: Register, val: RegisterValue<R,W>)-> Result<(), RegisterError> {
                match reg {
                    $(
                        Register::$register => match val {
                            RegisterValue::Word(_) => Err(RegisterError::InvalidGeneralRegisterValue),
                            // This will never panic as the general register array's length is calculated by the amount of general registers
                            RegisterValue::Other(other) => Ok(self.general[Register::$register as usize] += other),
                        },
                    )*
                    Register::PC => match val {
                        RegisterValue::Word(word) => Ok(self.pc += word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidProgramCounterValue),
                    },
                    Register::SP => match val {
                        RegisterValue::Word(word) => Ok(self.sp += word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidStackPointerValue),
                    }
                }
            }

            /// Subtract a value to a register.
            ///
            /// # Errors
            ///
            /// This function will return an error if the type of the value does not match the type of the register.
            #[inline]
            pub fn sub(&mut self, reg: Register, val: RegisterValue<R,W>)-> Result<(), RegisterError> {
                match reg {
                    $(
                        Register::$register => match val {
                            RegisterValue::Word(_) => Err(RegisterError::InvalidGeneralRegisterValue),
                            // This will never panic as the general register array's length is calculated by the amount of general registers
                            RegisterValue::Other(other) => Ok(self.general[Register::$register as usize] -= other),
                        },
                    )*
                    Register::PC => match val {
                        RegisterValue::Word(word) => Ok(self.pc -= word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidProgramCounterValue),
                    },
                    Register::SP => match val {
                        RegisterValue::Word(word) => Ok(self.sp -= word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidStackPointerValue),
                    }
                }
            }

            /// Multiply a value with the value in a register and assign the result back to the register.
            ///
            /// # Errors
            ///
            /// This function will return an error if the type of the value does not match the type of the register.
            #[inline]
            pub fn mul(&mut self, reg: Register, val: RegisterValue<R,W>)-> Result<(), RegisterError> {
                match reg {
                    $(
                        Register::$register => match val {
                            RegisterValue::Word(_) => Err(RegisterError::InvalidGeneralRegisterValue),
                            // This will never panic as the general register array's length is calculated by the amount of general registers
                            RegisterValue::Other(other) => Ok(self.general[Register::$register as usize] *= other),
                        },
                    )*
                    Register::PC => match val {
                        RegisterValue::Word(word) => Ok(self.pc *= word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidProgramCounterValue),
                    },
                    Register::SP => match val {
                        RegisterValue::Word(word) => Ok(self.sp *= word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidStackPointerValue),
                    }
                }
            }

            /// Divide the value of a register by another value and assign the result back to the register.
            ///
            /// # Errors
            ///
            /// This function will return an error if the type of the value does not match the type of the register.
            #[inline]
            pub fn div(&mut self, reg: Register, val: RegisterValue<R,W>)-> Result<(), RegisterError> {
                match reg {
                    $(
                        Register::$register => match val {
                            RegisterValue::Word(_) => Err(RegisterError::InvalidGeneralRegisterValue),
                            // This will never panic as the general register array's length is calculated by the amount of general registers
                            RegisterValue::Other(other) => Ok(self.general[Register::$register as usize] /= other),
                        },
                    )*
                    Register::PC => match val {
                        RegisterValue::Word(word) => Ok(self.pc /= word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidProgramCounterValue),
                    },
                    Register::SP => match val {
                        RegisterValue::Word(word) => Ok(self.sp /= word),
                        RegisterValue::Other(_) => Err(RegisterError::InvalidStackPointerValue),
                    }
                }
            }

            /// Increment the value in a register by one.
            #[inline]
            pub fn inc(&mut self, reg: Register) {
                match reg {
                    $(
                        // This will never panic as the general register array's length is calculated by the amount of general registers
                        Register::$register => self.general[Register::$register as usize] += 1.into(),
                    )*
                    Register::PC => {self.pc += 1.into()},
                    Register::SP => {self.sp +=1.into()}
                }
            }

            /// Decrement the value in a register by one.
            #[inline]
            pub fn dec(&mut self, reg: Register) {
                match reg {
                    $(
                        // This will never panic as the general register array's length is calculated by the amount of general registers
                        Register::$register => self.general[Register::$register as usize] -= 1.into(),
                    )*
                    Register::PC => self.pc -= 1.into(),
                    Register::SP => self.sp -=1.into()
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
