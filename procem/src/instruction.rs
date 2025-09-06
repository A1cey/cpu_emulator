use core::fmt::Debug;
use core::ops::Deref;

use crate::{processor::Processor, word::Word};

pub trait Instruction<W: Word>: Debug + Copy + Eq + Ord {
    /// This function is called when an instruction is executed by the processor.
    fn execute<const STACK_SIZE: usize, P: Deref<Target = [Self]>>(
        instruction: Self,
        processor: &mut Processor<STACK_SIZE, Self, P, W>,
    );
}
