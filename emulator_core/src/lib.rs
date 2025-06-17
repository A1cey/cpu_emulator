//! This library provides the [`Processor`](processor::Processor) structure, which emulates a real-world processor architecture.
//!
//! The design is loosely based on the ARM architecture.
//!
//! The processor operates by loading and executing an assembly [`Program`](program::Program). 
//! A [`Program`](program::Program) is a collection of assembly instructions that the processor iterates over and executes. 
//! The instruction set in use must implement the [`InstructionSet`](instruction_set::InstructionSet) trait.
//! A default instruction set is available via the [`assembler`](../assembler/index.html) crate.
//!
//! # Implementing Custom Instruction Sets
//! 
//! The crate provides a [`Word`](word::Word) trait.
//! This trait wraps the underlying type used as the processor’s word size, mimicking real-world architectures 
//! (e.g., [`I32`](word::I32) corresponds to a 32-bit architecture).
//! 
//! The [`Registers`](register::Registers) and [`Stack`](stack::Stack) use [`Word`](word::Word) as their data type.
//! 
//! The following signed integer types implement the [`Word`](word::Word) trait:
//! - [`I8`](word::I8)
//! - [`I16`](word::I16)
//! - [`I32`](word::I32)
//! - [`I64`](word::I64)
//! - [`I128`](word::I128)
//! - [`ISize`](word::ISize)
//! 
//! These types use two's complement representation, mirroring how real-world processor architectures work.
//! 
//! The processor’s [`Registers`](register::Registers), [`Flags`](register::Flag) and [`Stack`](stack::Stack) 
//! are directly accessible and modifiable through the [`Processor`](processor::Processor) structure.
//! 
//! ```
//! let r0 = processor.registers.get_reg(Register::R0);
//! processor.registers.set_reg(Register::R1, r0);
//! 
//! let overflow = processor.registers.get_flag(Flag::V);
//! ```

pub mod instruction_set;
pub mod processor;
pub mod program;
pub mod register;
pub mod stack;
pub mod word;
