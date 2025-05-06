use crate::instruction::Instruction;
use crate::program::{Program, ProgramError};
use crate::register::{RegisterSize, Registers};
use crate::stack::{Stack, Word};

/// Processor struct
#[derive(Debug)]
pub struct Processor<'a, R: RegisterSize, W: Word, const STACK_SIZE: usize> {
    pub registers: Registers<R, W>,
    pub stack: Stack<W, STACK_SIZE>,
    program: Option<&'a Program>,
}

impl<'a, R: RegisterSize, W: Word, const STACK_SIZE: usize> Processor<'a, R, W, STACK_SIZE> {
    /// Create a new processor instance
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            stack: Stack::new(),
            program: None,
        }
    }

    /// Load a program into the processor
    pub fn load_program(&mut self, program: &'a Program) {
        self.program = Some(program);
    }

    /// Execute the next instruction in the program
    pub fn execute_next_instruction(&mut self) -> Result<(), ProgramError> {
        let program = self.program.ok_or(ProgramError::NoProgramLoaded)?;

        let instruction = program.get_instruction(self.registers.pc.into())?;

        Instruction::execute(instruction, self);

        Ok(())
    }
}
