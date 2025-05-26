use asm_compiler::instruction_set::Instruction;
use emulator_core::{processor::Processor, program::Program};

fn integration_test() {
    let p = Processor::<1024, Instruction<1024>>::new();
    let pr = Program::new(Instruction::End);
    p.load_program(pr);
    p.execute_next_instruction();
}