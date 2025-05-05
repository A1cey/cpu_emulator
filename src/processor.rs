use crate::register::{Register8Bit, Register16Bit};
use crate::instruction::Instruction;

/// Z80 processor
struct Z80 {
    registers: Z80Registers,
    instructions: Vec<Instruction>,
}

impl Z80 {
    fn new(registers: Z80Registers, instructions: Vec<Instruction>) -> Self {
        Z80 { registers, instructions }
    }

    fn execute_instruction(&mut self)-> Result<(),String> {
        let instruction = self.get_next_instruction()?;
        
        match *instruction {
            Instruction::ADD => self.add(),
            Instruction::NOP
        }
        Ok(())
    }
    
    fn get_next_instruction(&mut self) -> Result<&Instruction, String> {
        let pc = &mut self.registers.PC;
        let instruction = self.instructions.get(*pc as usize).ok_or("Program counter out of bounds")?;
        *pc += 1;
        Ok(instruction)
    }
}

/// Z80 processor registers
/// https://www.zilog.com/docs/z80/um0080.pdf
#[allow(non_snake_case)]
struct Z80Registers {
    /// Accumulator registers
    A: Register8Bit,
    A1: Register8Bit,
    
    /// Flag registers, F corresponds to A and F1 corresponds to A1 accumulator
    F: Register8Bit,
    F1: Register8Bit,
    
    /// General purpose register pairs
    /// A registers pair can be used individually as two individual 8 bit registers or as a single 16 bit register
    B: Register8Bit,
    C: Register8Bit,
    
    D: Register8Bit,
    E: Register8Bit, 
    
    H: Register8Bit,
    L: Register8Bit,
    
    B1: Register8Bit,
    C1: Register8Bit,
    
    D1: Register8Bit,
    E1: Register8Bit,
    
    H1: Register8Bit,
    L1: Register8Bit,
    
    /// Interrupt vector register
    I: Register8Bit,
    
    /// Memory refresh register
    R: Register8Bit,
    
    /// Index registers
    IX: Register16Bit,
    IY: Register16Bit,
    
    /// Stack pointer
    SP: Register16Bit,
    
    /// Program counter
    PC: Register16Bit,
}
