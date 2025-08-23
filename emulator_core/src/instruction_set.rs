use core::fmt::Debug;
use std::ops::Deref;

use crate::{processor::Processor, word::Word};

/// Trait for implementing a instruction set that can be used by the processor.
pub trait InstructionSet: Sized {
    type Instruction: Debug + Clone + Copy + Eq + Ord;
    type W: Word;

    /// This function is called when an instruction is executed by the processor.
    fn execute<const STACK_SIZE: usize, P: Deref<Target = [Self::Instruction]>>(
        instruction: Self::Instruction,
        processor: &mut Processor<STACK_SIZE, Self, P>,
    );
}
