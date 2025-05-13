use thiserror::Error;

use crate::instruction::{ExecutionError, Instruction};
use crate::program::{Program, ProgramError};
use crate::register::{Register, Registers};
use crate::stack::{Stack, Word};

use core::ops::ControlFlow;

/// Processor struct
#[derive(Debug)]
pub struct Processor<'a, W: Word, const STACK_SIZE: usize> {
    pub registers: Registers<W>,
    pub stack: Stack<W, STACK_SIZE>,
    program: Option<&'a Program<W, STACK_SIZE>>,
}

impl<W: Word, const STACK_SIZE: usize> Default for Processor<'_, W, STACK_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, W: Word, const STACK_SIZE: usize> Processor<'a, W, STACK_SIZE> {
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
    pub fn load_program(&mut self, program: &'a Program<W, STACK_SIZE>) {
        self.program = Some(program);
    }

    /// Run the entire program.
    ///
    /// # Errors
    /// Returns Processor error if an error occured during execution.
    pub fn run_program(&mut self) -> Result<(), ProcessorError> {
        while self.execute_next_instruction()? == ControlFlow::Continue(()) {}
        Ok(())
    }

    /// Execute the current instruction in the program (where pc points to) and increment pc.
    ///
    /// # Errors
    /// Returns Processor error if an error occured during execution.
    pub fn execute_next_instruction(&mut self) -> Result<ControlFlow<()>, ProcessorError> {
        println!("{}", self.registers);

        let program = self.program.ok_or(ProgramError::NoProgramLoaded)?;

        let instruction = program.get_instruction(self.registers.pc.into())?;

        let res = Instruction::execute(instruction, self);

        self.registers.inc(Register::PC);

        Ok(res)
    }
}

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("Program error")]
    Program(#[from] ProgramError),
    #[error("Execution error")]
    Execution(#[from] ExecutionError),
}
