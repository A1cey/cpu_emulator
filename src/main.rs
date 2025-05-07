use instruction::Instruction;
use register::{Register, RegisterValue};
use stack::U16;

mod instruction;
mod processor;
mod program;
mod register;
mod stack;

fn main() {
    let instructions = vec![
        Instruction::MoveVal {
            to: Register::R0,
            val: RegisterValue::Other(5),
        },
        Instruction::MoveVal {
            to: Register::R1,
            val: RegisterValue::Other(5),
        },
        Instruction::AddReg {
            acc: Register::R0,
            rhs: Register::R1,
        },
        Instruction::MulReg {
            acc: Register::R0,
            rhs: Register::R1,
        },
    ];
    let program = program::Program::new(instructions);
    let mut processor = processor::Processor::<u16, U16, 1024>::new();
    processor.load_program(&program);
    let _ = processor.run_program();

    println!("{:#?}", processor.registers);
}
