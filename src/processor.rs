use thiserror::Error;

use crate::instruction::{ExecutionError, Instruction};
use crate::program::{Program, ProgramError};
use crate::register::{Register, RegisterSize, Registers};
use crate::stack::{Stack, Word};

/// Processor struct
#[derive(Debug)]
pub struct Processor<'a, R: RegisterSize, W: Word, const STACK_SIZE: usize> {
    pub registers: Registers<R, W>,
    pub stack: Stack<W, STACK_SIZE>,
    program: Option<&'a Program<R, W, STACK_SIZE>>,
}

impl<'a, R: RegisterSize, W: Word, const STACK_SIZE: usize> Processor<'a, R, W, STACK_SIZE> {
    /// Create a new processor instance.
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            stack: Stack::new(),
            program: None,
        }
    }

    /// Load a program into the processor.
    pub fn load_program(&mut self, program: &'a Program<R, W, STACK_SIZE>) {
        self.program = Some(program);
    }

    /// Run the entire program.
    pub fn run_program(&mut self) -> Result<(), ProcessorError> {
        loop {
            self.execute_next_instruction()?
        }
    }

    /// Execute the current instruction in the program (where pc points to) and increment pc.
    pub fn execute_next_instruction(&mut self) -> Result<(), ProcessorError> {
        let program = self.program.ok_or(ProgramError::NoProgramLoaded)?;

        let instruction = program.get_instruction(self.registers.pc.into())?;

        Instruction::execute(instruction, self)?;

        self.registers.inc(Register::PC);

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("Program error")]
    Program(#[from] ProgramError),
    #[error("Execution error")]
    Execution(#[from] ExecutionError),
}
