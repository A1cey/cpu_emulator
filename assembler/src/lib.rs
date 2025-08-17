use crate::parser::{Parser, ParserError};
use crate::tokenizer::{Tokenizer, TokenizerError};
use emulator_core::{program::Program, word::Word};
use thiserror::Error;

use crate::instruction_set::Instruction;

pub mod instruction_set;
mod parser;
mod tokenizer;

/// Assembles Program from Assembly Code.
///
/// # Errors
/// Returns a vector of all errors that a happened during either the tokenizing or the parsing.
pub fn assemble<const STACK_SIZE: usize, W: Word>(
    input: impl AsRef<str>,
) -> Result<Program<STACK_SIZE, Instruction<STACK_SIZE, W>>, Vec<AssemblerError>> {
    Tokenizer::tokenize(input.as_ref())
        .map_err(|err| err.into_iter().map(Into::into).collect())
        .and_then(|tokens| Parser::parse(&tokens).map_err(|err| err.into_iter().map(Into::into).collect()))
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AssemblerError {
    #[error("Error during parsing: ")]
    Parser(#[from] ParserError),
    #[error("Error during tokenization: ")]
    Tokenizer(#[from] TokenizerError),
}
