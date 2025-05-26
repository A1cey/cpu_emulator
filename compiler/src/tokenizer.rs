use emulator_core::stack::Word;

use crate::instruction_set::Instruction;

#[derive(Debug, Clone, PartialEq,Eq)]
enum Token<'a, const STACK_SIZE: usize, W: Word> {
    Label(&'a str),
    Register(&'a str),
    Literal(Literal<'a, W>),
    Instruction(Instruction<STACK_SIZE, W>)
}

// TODO: use Word as number 
#[derive(Debug, Clone, PartialEq,Eq)]
enum Literal<'a, W: Word> {
    Char(char),
    String(&'a str),
    Number(W),
    Boolean(bool)
}


fn tokenize<const STACK_SIZE: usize, W: Word, T>(input: &str) -> Vec<Token<STACK_SIZE, W>> {
    let mut tokens = vec![];
    let input = input.trim();
    
    
    if input.starts_with(".") {
       // expect_label()
    } else {
    //    expect_instruction()
    }
    
    tokens
}