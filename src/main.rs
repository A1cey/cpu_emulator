use instruction::Instruction;
use register::Register;
use stack::{I32, U32};

pub mod instruction;
pub mod processor;
pub mod program;
mod register;
mod stack;

fn main() {
    let instructions = vec![
        Instruction::SubVal {
            acc: Register::R0,
            val: 5.into(),
        },
        Instruction::MoveVal {
            to: Register::R1,
            val: 6.into(),
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
    let mut processor = processor::Processor::<I32, 1024>::new();
    processor.load_program(&program);
    if let Err(err) = processor.run_program() {
        println!("{:#?}", err);
    } else {
        println!("{}", processor.registers);
    }
}
