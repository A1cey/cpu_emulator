use std::error::Error;
use std::fmt::{self, Display};

use crate::instruction::Instruction;
use crate::program::Program;
use crate::register::{RegisterSize, Registers};
use crate::stack::{Stack, Word};

/// Processor struct
#[derive(Debug)]
pub struct Processor<'a, R: RegisterSize, W: Word, const STACK_SIZE: usize> {
    registers: Registers<R, W>,
    stack: Stack<W, STACK_SIZE>,
    program: &'a Program,
}

impl<'a, R: RegisterSize, W: Word, const STACK_SIZE: usize> Processor<'a, R, W, STACK_SIZE> {
    /// Create a new processor instance
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            stack: Stack::new(),
            program: &[],
        }
    }

    /// Load a program into the processor
    pub fn load_program(&mut self, program: &'a [Instruction]) {
        self.program = program;
    }
    
    /// Execute the next instruction in the program
    pub fn execute_instruction(&mut self)-> Result<(), String> {
    }

}

