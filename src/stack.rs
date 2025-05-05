use std::{
    error::Error,
    fmt::{self, Display},
    ops::{AddAssign, Deref, DerefMut},
};

/// Marker trait for types that can be used as words in a stack.
pub trait Word: Copy + Default + Sized + AddAssign {}

impl Word for u16 {}
impl Word for u32 {}
impl Word for u64 {}
impl Word for usize {}


/// Stack
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Stack<T: Word, const SIZE: usize>(pub [T; SIZE]);

impl<T: Word, const SIZE: usize> Deref for Stack<T, SIZE> {
    type Target = [T; SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Word, const SIZE: usize> DerefMut for Stack<T, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Word, const SIZE: usize> Stack<T, SIZE> {
    /// Create a new stack with all elements initialized to the default value.
    pub fn new() -> Self {
        Self([T::default(); SIZE])
    }

    /// Read a value from the stack at the given stack pointer.
    /// Returns the value on the stack or an OutOfBounds error.
    fn read(&self, sp: usize) -> Result<&T, OutOfBounds> {
        self.get(sp).ok_or(OutOfBounds)
    }

    /// Write a value to the stack at the given stack pointer.
    /// Returns an OutOfBounds error if the stack pointer is out of bounds.
    fn write(&mut self, sp: usize, value: T) -> Result<(), OutOfBounds> {
        *self.get_mut(sp).ok_or(OutOfBounds)? = value;
        Ok(())
    }
}

#[derive(Debug)]
struct OutOfBounds;

impl Error for OutOfBounds {}

impl Display for OutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Out of bounds stack access")
    }
}
