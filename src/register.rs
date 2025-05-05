use std::ops::AddAssign;

use crate::stack::Word;

/// Marker trait for register sizes.
pub trait RegisterSize: Copy + Default + Sized + AddAssign {}

impl RegisterSize for u8 {}
impl RegisterSize for u16 {}
impl RegisterSize for u32 {}
impl RegisterSize for u64 {}
impl RegisterSize for u128 {}
impl RegisterSize for usize {}

/// Registers
// TODO: User defined register count and sizes 
// TODO: Consider changing registers to use a array for the registers and a enum for the names
#[derive(Debug, PartialEq, Eq)]
pub struct Registers<R: RegisterSize, W: Word> {
    pub r0: R,
    pub r1: R,
    pub r2: R,
    pub r3: R,
    pub r4: R,
    pub r5: R,
    pub r6: R,
    pub r7: R,
    pub r8: R,
    pub r9: R,
    pub r10: R,
    pub r11: R,
    pub r12: R,
    pub r13: R,
    pub r14: R,
    pub r15: R,
    pub pc: W,
    pub sp: W,
}

impl<R: RegisterSize, W: Word> Registers<R, W> {
    /// Create a new set of registers with all values initialized to the default value.
    pub fn new() -> Self {
        Registers {
            r0: R::default(),
            r1: R::default(),
            r2: R::default(),
            r3: R::default(),
            r4: R::default(),
            r5: R::default(),
            r6: R::default(),
            r7: R::default(),
            r8: R::default(),
            r9: R::default(),
            r10: R::default(),
            r11: R::default(),
            r12: R::default(),
            r13: R::default(),
            r14: R::default(),
            r15: R::default(),
            pc: W::default(),
            sp: W::default(),
        }
    }
}
