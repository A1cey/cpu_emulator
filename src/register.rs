use core::fmt::Debug;
use thiserror::Error;

use crate::stack::Word;

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
        /// The register sizes correspond to the Stack Word size.
        #[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
        pub struct Registers<W> {
            pub general: [W; COUNT],
            pub pc: W,
            pub sp: W,
        }

        impl<W: Word> Registers<W> {
            /// Create a new set of registers with all values initialized to the default value.
            pub fn new() -> Self {
                Registers {
                    general: [W::default(); COUNT],
                    pc: W::default(),
                    sp: W::default(),
                }
            }

            /// Get the value of a register.
            #[inline]
            pub fn get(&self, reg: Register) -> W {
                match reg {
                    $(
                        // This will never panic as the general register array's length is calculated by the amount of general registers
                        Register::$register => self.general[Register::$register as usize],
                    )*
                    Register::PC => self.pc,
                    Register::SP => self.sp,
                }
            }

            /// Set the value of a register.
            #[inline]
            pub fn set(&mut self, reg: Register, val: W) {
                match reg {
                    $(
                        Register::$register => self.general[Register::$register as usize] = val,
                    )*
                    Register::PC => self.pc = val,
                    Register::SP => self.sp = val
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
                    Register::SP => {self.sp += 1.into()}
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

            fn fmt_general_registers(&self) -> String {
                let s:String  = self.general.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
                format!("[{s}]")
            }
        }


        impl<W: Word> ::core::fmt::Display for Registers<W> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error> {
                write!(f, "general:\t{}\npc:\t\t{}\nsp:\t\t{}", self.fmt_general_registers(), self.pc, self.sp)
            }
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
pub enum RegisterError {}
