use core::num::ParseIntError;
use std::collections::HashMap;

use emulator_core::{
    program::Program,
    register::{Register, RegisterError},
    stack::Word,
};
use thiserror::Error;

use crate::{
    instruction_set::{ASMBinaryInstruction, ASMUnaryInstruction, Instruction},
    tokenizer::{Literal, Token},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct Parser<'a, const STACK_SIZE: usize, W: Word> {
    tokens: &'a [Token<'a>],
    instructions: Vec<Instruction<STACK_SIZE, W>>,
    errors: Option<Vec<ParserError>>,
    idx: usize,
    labels: HashMap<&'a str, usize>,
    is_done: bool,
}

impl<'a, const STACK_SIZE: usize, W: Word> Parser<'a, STACK_SIZE, W> {
    fn new(tokens: &'a [Token<'a>]) -> Self {
        Self {
            tokens,
            errors: None,
            instructions: Default::default(),
            idx: 0,
            labels: Default::default(),
            is_done: false,
        }
    }

    pub(crate) fn parse(
        tokens: &'a [Token<'a>],
    ) -> Result<Program<STACK_SIZE, Instruction<STACK_SIZE, W>>, Vec<ParserError>> {
        let mut parser = Parser::<STACK_SIZE, W>::new(tokens);
        parser.run();

        match parser.errors {
            None => Ok(Program::new(parser.instructions)),
            Some(err) => Err(err),
        }
    }

    fn run(&mut self) {
        while self.idx < self.tokens.len() && !self.is_done {
            match self.tokens.get(self.idx) {
                Some(token) => match token {
                    Token::Label(label) => {
                        if let Some(old_idx) = self.labels.insert(&label, self.idx) {
                            self.add_error(ParserError::DuplicateLabel {
                                idx: self.idx,
                                old_idx,
                            });
                        }
                    }
                    Token::Instruction(inst) => self.parse_instruction(inst),
                    Token::End => self.is_done = true,
                    token => self.add_error(ParserError::InvalidToken {
                        idx: self.idx,
                        expected: "Label or Instruction",
                        got: format!("{token:?}"),
                    }),
                },
                None => self.is_done = true,
            }

            self.idx += 1;
        }
    }

    #[inline]
    fn add_error(&mut self, err: ParserError) {
        self.errors.get_or_insert_default().push(err);
    }

    fn parse_instruction(&mut self, inst: &str) {
        match inst {
            "NOP" => self.instructions.push(Instruction::Nop),
            "ADD" => self.expect_binary_instruction(ASMBinaryInstruction::Add),
            "SUB" => self.expect_binary_instruction(ASMBinaryInstruction::Sub),
            "MUL" => self.expect_binary_instruction(ASMBinaryInstruction::Mul),
            "DIV" => self.expect_binary_instruction(ASMBinaryInstruction::Div),
            "MOV" => self.expect_binary_instruction(ASMBinaryInstruction::Mov),
            "JMP" => self.expect_destination(),
            "INC" => self.expect_unary_instruction(ASMUnaryInstruction::Inc),
            "DEC" => self.expect_unary_instruction(ASMUnaryInstruction::Dec),
            _ => self.add_error(ParserError::UnknownInstruction {
                idx: self.idx,
                inst: inst.to_string(),
            }),
        }
    }

    fn expect_destination(&mut self) {
        self.idx += 1;

        match self.tokens.get(self.idx) {
            Some(Token::Label(label)) => match self.labels.get(label.as_str()) {
                Some(idx) => self.instructions.push(Instruction::Jump {
                    to: (*idx as i32).into(),
                }),
                None => self.add_error(ParserError::LabelNotFound {
                    idx: self.idx,
                    label: label.clone(),
                }),
            },
            token => {
                let token_str = match token {
                    Some(token) => format!("{:?}", token),
                    None => String::new(),
                };
                self.add_error(ParserError::InvalidToken {
                    idx: self.idx,
                    expected: "Label",
                    got: token_str,
                })
            }
        }
    }

    fn expect_register(&mut self) -> Result<Register, ParserError> {
        match self.get_next() {
            Token::Register(reg) => reg
                .parse::<Register>()
                .map_err(|err| ParserError::RegisterParsing(err)),
            token => {
                let token_str = format!("{:?}", token);
                Err(ParserError::InvalidToken {
                    idx: self.idx,
                    expected: "Register",
                    got: token_str,
                })
            }
        }
    }

    #[inline]
    fn get_next(&mut self) -> &Token<'_> {
        self.idx += 1;
        self.tokens.get(self.idx).unwrap_or(&Token::End)
    }

    fn convert_lit_to_val(&self, lit: &Literal<'_>) -> Result<W, ParserError> {
        match lit {
            Literal::Char(s) => Ok((*s as i32).into()),
            Literal::Binary(s) => {
                W::from_str_radix(s, 2).map_err(|err| ParserError::LiteralParsing(err))
            }
            Literal::Boolean(s) => Ok((if *s { 1 } else { 0 }).into()),
            Literal::Decimal(s) => {
                W::from_str_radix(s, 10).map_err(|err| ParserError::LiteralParsing(err))
            }
            Literal::Hexadecimal(s) => {
                W::from_str_radix(s, 16).map_err(|err| ParserError::LiteralParsing(err))
            }
            Literal::Octal(s) => {
                W::from_str_radix(s, 8).map_err(|err| ParserError::LiteralParsing(err))
            }
            Literal::String(_) => Err(ParserError::CannotConvertStrToVal),
        }
    }

    fn expect_binary_instruction(&mut self, inst: ASMBinaryInstruction) {
        let acc = match self.expect_register() {
            Ok(reg) => reg,
            Err(err) => {
                return self.add_error(err);
            }
        };

        self.idx += 1;
        match self.tokens.get(self.idx) {
            Some(Token::Comma) => {}
            token => {
                let token_str = match token {
                    Some(token) => format!("{:?}", token),
                    None => String::new(),
                };
                self.add_error(ParserError::InvalidToken {
                    idx: self.idx,
                    expected: "Comma",
                    got: token_str,
                })
            }
        }

        self.idx += 1;
        match self.tokens.get(self.idx) {
            Some(Token::Register(rhs)) => {
                let rhs = match rhs.parse::<Register>() {
                    Ok(reg) => reg,
                    Err(err) => {
                        return self.add_error(ParserError::RegisterParsing(err));
                    }
                };
                self.instructions
                    .push(Instruction::from_binary_reg_instr(inst, acc, rhs))
            }
            Some(Token::Literal(lit)) => {
                let val = match self.convert_lit_to_val(lit) {
                    Ok(val) => val,
                    Err(err) => {
                        return self.add_error(err);
                    }
                };

                self.instructions
                    .push(Instruction::from_binary_val_instr(inst, acc, val))
            }
            token => {
                let token_str = match token {
                    Some(token) => format!("{:?}", token),
                    None => String::new(),
                };
                self.add_error(ParserError::InvalidToken {
                    idx: self.idx,
                    expected: "Register or Literal",
                    got: token_str,
                })
            }
        }
    }

    fn expect_unary_instruction(&mut self, inst: ASMUnaryInstruction) {
        let reg = match self.expect_register() {
            Ok(reg) => reg,
            Err(err) => {
                return self.add_error(err);
            }
        };

        self.instructions
            .push(Instruction::from_unary_instr(inst, reg));
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ParserError {
    #[error("No tokens to parse.")]
    EmptyTokenList,
    #[error("Invalid token at idx {idx}. Expected: {expected} Got: {got}")]
    InvalidToken {
        idx: usize,
        expected: &'static str,
        got: String,
    },
    #[error("Duplicate lable: First occurrence: {old_idx}, second occurrence {idx}")]
    DuplicateLabel { idx: usize, old_idx: usize },
    #[error("Unkown instruction at idx {idx}: {inst}")]
    UnknownInstruction { idx: usize, inst: String },
    #[error("Error while parsing register.")]
    RegisterParsing(#[from] RegisterError),
    #[error("Error while parsing literal.")]
    LiteralParsing(#[from] ParseIntError),
    #[error(
        "Strings cannot be converted to numeric values directly. You could use a hex representation instead."
    )]
    CannotConvertStrToVal,
    #[error("Label \".{label}\" not found. Needed at {idx}.")]
    LabelNotFound { idx: usize, label: String },
}
