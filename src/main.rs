use instruction::Instruction;
use stack::U16;

mod instruction;
mod processor;
mod program;
mod register;
mod stack;

fn main() {
    let instructions = vec![Instruction::Nop];
    let program = program::Program::new(instructions);
    let mut processor = processor::Processor::<u16, U16, 1024>::new();
    processor.load_program(&program);
    let _ = processor.execute_next_instruction();
}
