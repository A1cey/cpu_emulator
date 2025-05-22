use core::{fmt::Debug, ops::ControlFlow};

use crate::{processor::Processor, stack::Word};

/// The default_instruction_set module contains the default instruction set for the processor.
pub mod default_instruction_set;

/// Trait for implementing a instruction set that can be used by the processor.
pub trait InstructionSet<const STACK_SIZE: usize>: Sized {
    type Instruction: Debug + Copy + Eq;
    type W: Word;

    /// This function is called when an instruction is executed by the processor.
    fn execute(
        instruction: &Self::Instruction,
        processor: &mut Processor<STACK_SIZE, Self>,
    ) -> ControlFlow<()>;
}
