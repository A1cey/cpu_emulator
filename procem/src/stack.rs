//! The processor's [`Stack`].

use crate::helper;
use crate::word::Word;
use core::fmt::{Debug, Display, Formatter};
use core::ops::{Deref, DerefMut};

/// The [`Stack`] is a wrapper around a fixed-size array of values implementing the [`Word`] trait.
/// It can be read with the [`read`](Stack::read) method. It can also be written to with the [`write`](Stack::write) method.
/// For both reading and writing, the stack pointer needs to be provided.
/// ```
/// # use procem::register::{Flag, Register};
/// # use procem::processor::Processor;
/// # use procem::instruction::Instruction;
/// # use procem::word::{I64, Word};
/// # use core::marker::PhantomData;
/// # use core::ops::Deref;
/// #
/// # #[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
/// # struct Inst<W: Word> (PhantomData<W>);
/// #
/// # impl<W: Word> Instruction<W> for Inst<W> {
/// #     fn execute<const STACK_SIZE: usize, P: Deref<Target = [Self]>>(
/// #         instruction: Self,
/// #         processor: &mut Processor<STACK_SIZE, Self, P, W>
/// #     ) {}
/// # }
/// # let mut processor = Processor::<4, _,  Vec<Inst<I64>>,_>::new();
/// // Default stack values are all zero.
/// assert_eq!(processor.stack.read(processor.registers.get_reg(Register::SP)), 0.into());
/// 
/// processor.stack.write(processor.registers.get_reg(Register::SP), 1.into());
/// assert_eq!(processor.stack.read(processor.registers.get_reg(Register::SP)), 1.into());
/// 
/// processor.registers.inc(Register::SP);
/// processor.stack.write(processor.registers.get_reg(Register::SP), 10.into());
/// assert_eq!(processor.stack.read(processor.registers.get_reg(Register::SP)), 10.into());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Stack<const STACK_SIZE: usize, W: Word>([W; STACK_SIZE]);

impl<const STACK_SIZE: usize, W: Word> Deref for Stack<STACK_SIZE, W> {
    type Target = [W; STACK_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const STACK_SIZE: usize, W: Word> DerefMut for Stack<STACK_SIZE, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const STACK_SIZE: usize, W: Word> Default for Stack<STACK_SIZE, W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const STACK_SIZE: usize, W: Word> Display for Stack<STACK_SIZE, W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{}", helper::FmtArray(self.deref().as_slice()))
    }
}

impl<const STACK_SIZE: usize, W: Word> Stack<STACK_SIZE, W> {
    /// Create a new stack with all elements initialized to the default value.
    #[must_use]
    pub fn new() -> Self {
        Self([W::default(); STACK_SIZE])
    }

    /// Read a value from the stack at the given stack pointer.
    ///
    /// # Panics
    /// Panics if the stack pointer is out of bounds.
    pub fn read(&self, sp: W) -> W {
        self.get(sp.into())
            .copied()
            .unwrap_or_else(|| panic!("Out of bounds stack access. Stack size: {STACK_SIZE}, Stack pointer: {sp}"))
    }

    /// Write a value to the stack at the given stack pointer.
    ///
    /// # Panics
    /// Panics if the stack pointer is out of bounds.
    pub fn write(&mut self, sp: W, value: W) {
        *self
            .get_mut(sp.into())
            .unwrap_or_else(|| panic!("Out of bounds stack access. Stack size: {STACK_SIZE}, Stack pointer: {sp}")) =
            value;
    }
}
