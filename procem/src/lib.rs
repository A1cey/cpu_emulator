//! Procem provides a [`Processor`](processor::Processor) structure, which emulates a real-world processor architecture.
//!
//! The design is loosely based on the ARM architecture.
//!
//! The processor operates by loading and executing an assembly [`Program`](program::Program).
//! A [`Program`](program::Program) is a collection of assembly instructions that the processor iterates over and executes.
//! The instruction set in use must implement the [`Instruction`](instruction::Instruction) trait.
//! A default instruction set is available in the [`procem_default`](../procem_default/index.html) crate.
//!
//! # The [`Word`](word::Word) trait
//!
//! Procem provides a [`Word`](word::Word) trait.
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
//! To implement custom [`Word`](word::Word) types, you can define your own type that implements the [`Word`](word::Word) trait.
//! 
//! 
//! The processor’s [`Registers`](register::Registers), [`Flags`](register::Flag) and [`Stack`](stack::Stack)
//! are directly accessible and modifiable through the [`Processor`](processor::Processor) structure.
//!
//! ```
//! # use procem::register::{Flag, Register};
//! # use procem::processor::Processor;
//! # use procem::instruction::Instruction;
//! # use procem::word::{I32, Word};
//! # use core::marker::PhantomData;
//! # use core::ops::Deref;
//! #
//! # #[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
//! # struct Inst<W: Word> (PhantomData<W>);
//! #
//! # impl<W: Word> Instruction<W> for Inst<W> {
//! #     fn execute<const STACK_SIZE: usize, P: Deref<Target = [Self]>>(
//! #         instruction: Self,
//! #         processor: &mut Processor<STACK_SIZE, Self, P, W>
//! #     ) {}
//! # }
//! #
//! # let mut processor = Processor::<2048, _, Vec<Inst<I32>>, _>::new();
//! let r0 = processor.registers.get_reg(Register::R0);
//! processor.registers.set_reg(Register::R1, r0);
//!
//! let overflow = processor.registers.get_flag(Flag::V);
//! ```
//! 
//! 

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod instruction;
pub mod processor;
pub mod program;
pub mod register;
pub mod stack;
pub mod word;

mod helper;
