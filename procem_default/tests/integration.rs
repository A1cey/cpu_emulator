use procem::{processor::Processor, program::Program, register::Register, word::I32};
use procem_default::{
    AssemblerError, assemble,
    instruction::{Instruction, jump_condition::JumpCondition, operand::Operand},
    parser::ParserError,
};

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
        Program::<IS, Vec<Instruction<I32>>, I32>::new(vec![
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

    let mut processor = Processor::<STACK_SIZE, _, _, _>::builder()
        .with_program(&program)
        .build();

    println!("{processor}");

    for _ in 0..14 {
        assert!(processor.execute_next_instruction().is_ok());
    }

    assert_eq!(processor.registers.get_reg(Register::R1), 10.into());
    assert_eq!(processor.registers.pc(), 2.into());

    assert!(processor.execute_next_instruction().is_ok());
    assert_eq!(processor.registers.pc(), 0.into());
}

#[test]
fn parse_various_literals() {
    let program = assemble::<I32>(
        "
        mov R0, #42
        mov R1, #0b101010
        mov R2, #0x2A
        mov R3, #0o52
        mov R4, #true
        mov R5, #false
        mov R6, #'A'
        ",
    )
    .unwrap();

    assert_eq!(program.len(), 7);
    assert_eq!(
        program,
        Program::from(vec![
            Instruction::Mov {
                to: Register::R0,
                from: Operand::Value(42.into())
            },
            Instruction::Mov {
                to: Register::R1,
                from: Operand::Value(42.into())
            },
            Instruction::Mov {
                to: Register::R2,
                from: Operand::Value(42.into())
            },
            Instruction::Mov {
                to: Register::R3,
                from: Operand::Value(42.into())
            },
            Instruction::Mov {
                to: Register::R4,
                from: Operand::Value(1.into())
            },
            Instruction::Mov {
                to: Register::R5,
                from: Operand::Value(0.into())
            },
            Instruction::Mov {
                to: Register::R6,
                from: Operand::Value(65.into())
            }
        ])
    )
}

#[test]
fn parse_and_execute_arithmetic() {
    let program = assemble::<I32>(
        "
        mov R0, #10
        mov R1, #5
        add R0, R1
        sub R0, #3
        mul R0, #2
        div R0, #4
        ",
    )
    .unwrap();

    let mut processor = Processor::<1024, _, _, _>::builder().with_program(&program).build();

    let _ = processor.run_program();

    assert_eq!(processor.registers.get_reg(Register::R0), 6.into());
}

#[test]
fn control_flow_and_labels() {
    // Loop should run 5 times, incrementing R0 from 0 to 5
    let program = assemble::<I32>(
        "
        mov R0, #0
        mov R1, #5
        .loop
        add R0, #1
        subs R1, #1
        jnz .loop
        ",
    )
    .unwrap();

    let mut processor = Processor::<1024, _, _, _>::builder().with_program(&program).build();

    let _ = processor.run_program();
    assert_eq!(processor.registers.get_reg(Register::R0), 5.into());
}

#[test]
fn test_overflow_and_flags() {
    let program = assemble::<I32>(
        "
        mov R0, #2147483647
        add R0, #1
        cmp R0, #-2147483648
        ",
    )
    .unwrap();

    let mut processor = Processor::<1024, _, _, _>::builder().with_program(&program).build();

    let _ = processor.run_program();
    assert_eq!(processor.registers.get_reg(Register::R0), i32::MIN.into());
    assert_eq!(processor.registers.get_flag(procem::register::Flag::Z), true);
}

#[test]
fn factorial_program() {
    let program = assemble::<I32>(
        "
        mov R0, #5
        mov R1, #1
        .loop
        mul R1, R0
        subs R0, #1
        jnz .loop
        ",
    )
    .unwrap();

    let mut processor = Processor::<1024, _, _, _>::builder().with_program(&program).build();

    let _ = processor.run_program();
    assert_eq!(processor.registers.get_reg(Register::R1), 120.into());
}

#[test]
fn invalid_assembly_should_fail() {
    let result = assemble::<I32>("mov R0, #\"notanumber\"");

    assert_eq!(
        result,
        Err(vec![AssemblerError::Parser(ParserError::CannotConvertStrToVal)])
    );
}
