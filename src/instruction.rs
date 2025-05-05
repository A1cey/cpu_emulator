use crate::register::Register8Bit;

/// simplified Z80 instruction set 
/// https://www.zilog.com/docs/z80/um0080.pdf
pub enum Z80InstructionSet {
    Load(Register8Bit, u8),
    ADD,
    SUB,
    JUMP,
}