use assembler::{
    assemble,
    instruction::{Instruction, jump_condition::JumpCondition, operand::Operand},
};
use emulator_core::{processor::Processor, program::Program, register::Register, word::I32};

#[test]
fn simple_5x2_multiplication() {
    const STACK_SIZE: usize = 1024;
    type IS = Instruction<I32>;
    
    let program = assemble::<I32>(
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
        Program::<IS, Vec<Instruction<I32>>>::new(vec![
            Instruction::Mov {
                to: Register::R0,
                from: Operand::Value(2.into())
            },
            Instruction::Add {
                acc: Register::R1,
                rhs: Operand::Register(Register::R0),
                signed: false
            },
            Instruction::Jump {
                to: 0.into(),
                condition: JumpCondition::Unconditional
            }
        ])
    );

    let mut processor = Processor::<STACK_SIZE, _, _>::builder().with_program(&program).build();

    println!("{processor}");

    for _ in 0..14 {
        assert!(processor.execute_next_instruction().is_ok());
    }

    assert_eq!(processor.registers.get_reg(Register::R1), 10.into());
    assert_eq!(processor.registers.pc, 2.into());

    assert!(processor.execute_next_instruction().is_ok());
    assert_eq!(processor.registers.pc, 0.into());
}
