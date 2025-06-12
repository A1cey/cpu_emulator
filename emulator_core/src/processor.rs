//! The [`Processor`] is the main component of the emulator. It represents a simplified real world processor with a stack, registers and flags.
//!
//! It can store a singular program.
//! It has 16 general use registers, a program counter (pc), a stack pointer (sp) and 4 flags (carry flag (C), signed flag (S), overflow flag (V), zero condition flag (Z)).
//! It also has a stack of size `STACK_SIZE`.
//!
//! To run a loaded program two methods are provided:
//! - To load a program use [`load_program()`](Processor::load_program()).
//! - To run the entire program use [`run_program()`](Processor::run_program()).
//! - To run only the next instruction use [`execute_next_instruction()`](Processor::execute_next_instruction()).
use crate::instruction_set::InstructionSet;
use crate::program::{Program, ProgramError};
use crate::register::{Register, Registers};
use crate::stack::Stack;

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
    #[must_use]
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            stack: Stack::new(),
            program: None,
        }
    }

    /// Loads a program into the processor.
    ///
    /// The program cannot be changed after being loaded. To make changes, an updated or entirely new program has to be loaded.
    pub const fn load_program(&mut self, program: &'a Program<STACK_SIZE, IS>) {
        self.program = Some(program);
    }

    /// Runs the entire program.
    ///
    /// # Errors
    /// The execution of the program stops and a `ProgramError` is returned if an error occured during the fetching of an instruction.
    ///
    /// Note: The execution of an instruction will never return an error. If the instruction is valid it will not error.
    /// Invalid instructions are a major bug in the implementation of the instruction set that is used for the program.
    pub fn run_program(&mut self) -> Result<(), ProgramError> {
        loop {
            self.execute_next_instruction()?;
        }
    }

    /// Fetches the current instruction (where pc points to), increments the pc and then executes the instruction.
    ///
    /// # Errors
    /// Returns a `ProgramError` if an error occured during fetching.
    ///
    /// Note: The execution of an instruction will never return an error. If the instruction is valid it will not error.
    /// Invalid instructions are a major bug in the implementation of the instruction set that is used for the program.
    pub fn execute_next_instruction(&mut self) -> Result<(), ProgramError> {
        println!("{}", self.registers);

        let program = self.program.ok_or(ProgramError::NoProgramLoaded)?;

        let instruction = program.fetch_instruction(self.registers.pc.into())?;

        self.registers.inc(Register::PC);

        IS::execute(instruction, self);

        Ok(())
    }
}
