#![warn(incomplete_features)]
#![feature(generic_const_exprs)]
use instruction::Instruction;
use register::Register;
use stack::{I16, I32, I8};

pub mod instruction;
pub mod processor;
pub mod program;
mod register;
mod stack;

fn main() {
    let instructions = vec![
        Instruction::MoveVal {
            to: Register::R0,
            val: 128.into(),
        },
        Instruction::MoveVal {
            to: Register::R1,
            val: 128.into(),
        },
        Instruction::AddReg {
            acc: Register::R0,
            rhs: Register::R1,
        },
        Instruction::MulReg {
            acc: Register::R0,
            rhs: Register::R1,
        },
        Instruction::End,
    ];
    let program = program::Program::new(instructions);
    let mut processor = processor::Processor::<Instruction<I8, 1024>>::new();
    processor.load_program(&program);
    if let Err(err) = processor.run_program() {
        println!("{err:#?}");
    } else {
        println!("{}", processor.registers);
    }
}
