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
use std::ops::Deref;

use crate::instruction_set::InstructionSet;
use crate::program::{Program, ProgramError};
use crate::register::{Register, Registers};
use crate::stack::Stack;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Processor<'a, const STACK_SIZE: usize, IS, P>
where
    IS: InstructionSet,
    P: Deref<Target = [IS::Instruction]>,
{
    pub registers: Registers<IS::W>,
    pub stack: Stack<STACK_SIZE, IS>,
    program: Option<&'a Program<IS, P>>,
}

impl<'a, const STACK_SIZE: usize, IS, P> Processor<'a, STACK_SIZE, IS, P>
where
    IS: InstructionSet,
    P: Deref<Target = [IS::Instruction]>,
{
    #[must_use]
    #[inline]
    pub fn builder() -> ProcessorBuilder<'a, STACK_SIZE, IS, P> {
        ProcessorBuilder::new()
    }

    /// Creates a new processor.
    #[must_use]
    #[inline]
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
    #[inline]
    pub fn load_program(&mut self, program: &'a Program<IS, P>) {
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
        let program = self.program.as_ref().ok_or(ProgramError::NoProgramLoaded)?;

        let instruction = program.fetch_instruction(self.registers.pc.into())?;

        self.registers.inc(Register::PC);

        IS::execute(instruction, self);

        Ok(())
    }
}

impl<'a, const STACK_SIZE: usize, IS, P> Default for Processor<'a, STACK_SIZE, IS, P>
where
    IS: InstructionSet,
    P: Deref<Target = [IS::Instruction]>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct ProcessorBuilder<'a, const STACK_SIZE: usize, IS, P>
where
    IS: InstructionSet,
    P: Deref<Target = [IS::Instruction]>,
{
    registers: Option<Registers<IS::W>>,
    stack: Option<Stack<STACK_SIZE, IS>>,
    program: Option<&'a Program<IS, P>>,
}

impl<'a, const STACK_SIZE: usize, IS, P> ProcessorBuilder<'a, STACK_SIZE, IS, P>
where
    IS: InstructionSet,
    P: Deref<Target = [IS::Instruction]>,
{
    /// Creates a new `ProcessorBuilder` with registers, stack and program set to `None`.
    #[inline]
    fn new() -> Self {
        Self {
            registers: None,
            stack: None,
            program: None,
        }
    }

    /// Sets the registers for the `ProcessorBuilder`.
    #[must_use]
    #[inline]
    pub fn with_registers(mut self, registers: Registers<IS::W>) -> Self {
        self.registers = Some(registers);
        self
    }

    /// Sets the stack for the `ProcessorBuilder`.
    #[must_use]
    #[inline]
    pub fn with_stack(mut self, stack: Stack<STACK_SIZE, IS>) -> Self {
        self.stack = Some(stack);
        self
    }

    /// Sets the program for the `ProcessorBuilder`.
    #[must_use]
    #[inline]
    pub fn with_program(mut self, program: &'a Program<IS, P>) -> Self {
        self.program = Some(program);
        self
    }

    /// Builds the `Processor` with the given registers, stack and program.
    #[must_use]
    #[inline]
    pub fn build(self) -> Processor<'a, STACK_SIZE, IS, P> {
        Processor {
            registers: self.registers.unwrap_or_default(),
            stack: self.stack.unwrap_or_default(),
            program: self.program,
        }
    }
}
