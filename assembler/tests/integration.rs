use assembler::{assemble, instruction_set::Instruction};
use emulator_core::{processor::Processor, program::Program, register::Register, word::I32};

#[test]
fn simple_5x2_multiplication() {
    const STACK_SIZE: usize = 1024;
    type IS = Instruction<STACK_SIZE, I32>;

    let program = assemble::<STACK_SIZE, I32>(
        "
        .input
        mov R0, #2
        add R1, R0
        jmp .input
        ",
    )
    .unwrap();

    assert_eq!(
        program,
        Program::<STACK_SIZE, IS>::new(vec![
            Instruction::MoveVal {
                to: Register::R0,
                val: 2.into()
            },
            Instruction::AddReg {
                acc: Register::R1,
                rhs: Register::R0
            },
            Instruction::Jump { to: 0.into() }
        ])
    );

    let mut processor = Processor::new();

    processor.load_program(&program);

    for _ in 0..14 {
        assert!(processor.execute_next_instruction().is_ok());
    }

    assert_eq!(processor.registers.get_reg(Register::R1), 10.into());
    assert_eq!(processor.registers.pc, 2.into());

    assert!(processor.execute_next_instruction().is_ok());
    assert_eq!(processor.registers.pc, 0.into());
}
