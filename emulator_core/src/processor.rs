use thiserror::Error;

use crate::instruction_set::InstructionSet;
use crate::program::{Program, ProgramError};
use crate::register::{Register, Registers};
use crate::stack::Stack;

/// Processor struct
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Processor<'a, const STACK_SIZE: usize, IS: InstructionSet<STACK_SIZE>> {
    pub registers: Registers<IS::W>,
    pub stack: Stack<STACK_SIZE, IS>,
    program: Option<&'a Program<STACK_SIZE, IS>>,
}

impl<const STACK_SIZE: usize, IS: InstructionSet<STACK_SIZE>> Default
    for Processor<'_, STACK_SIZE, IS>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, const STACK_SIZE: usize, IS: InstructionSet<STACK_SIZE>> Processor<'a, STACK_SIZE, IS> {
    /// Create a new processor instance.
    #[must_use]
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            stack: Stack::new(),
            program: None,
        }
    }

    /// Load a program into the processor.
    pub fn load_program(&mut self, program: &'a Program<STACK_SIZE, IS>) {
        self.program = Some(program);
    }

    /// Run the entire program.
    ///
    /// # Errors
    /// Returns ProcessorError if an error occured during execution.
    pub fn run_program(&mut self) -> Result<(), ProcessorError> {
        loop {
            self.execute_next_instruction()?
        }
    }

    /// Execute the current instruction in the program (where pc points to) and increment pc.
    ///
    /// # Errors
    /// Returns ProcessorError if an error occured during execution.
    pub fn execute_next_instruction(&mut self) -> Result<(), ProcessorError> {
        println!("{}", self.registers);

        let program = self.program.ok_or(ProgramError::NoProgramLoaded)?;

        let instruction = program.get_instruction(self.registers.pc.into())?;

        if IS::execute(instruction, self) == PCAutoIncrement::Active {
            self.registers.inc(Register::PC);
        }

        Ok(())
    }
}

/// Control enum for auto incrementing of PC after instruction execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PCAutoIncrement {
    Active,
    Inactive,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ProcessorError {
    #[error("Program error")]
    Program(#[from] ProgramError),
}
