use crate::instruction::Instruction;
use crate::parser::{Parser, ParserError};
use crate::tokenizer::{Tokenizer, TokenizerError};
use emulator_core::program::Program;
use emulator_core::word::Word;
use thiserror::Error;

pub mod instruction;
mod parser;
mod tokenizer;

pub type AssembledProgram<W> = Program<Instruction<W>, Vec<Instruction<W>>, W>;

/// Assembles Program from Assembly Code.
///
/// # Errors
/// Returns a vector of all errors that a happened during either the tokenizing or the parsing.
pub fn assemble<W: Word>(input: impl AsRef<str>) -> Result<AssembledProgram<W>, Vec<AssemblerError>> {
    let tokens = Tokenizer::tokenize(input.as_ref())
        .map_err(|err| err.into_iter().map(Into::into).collect::<Vec<AssemblerError>>())?;

    let instructions = Parser::parse(tokens.as_ref())
        .map_err(|err| err.into_iter().map(Into::into).collect::<Vec<AssemblerError>>())?;

    Ok(Program::new(instructions))
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AssemblerError {
    #[error("Error during parsing: ")]
    Parser(#[from] ParserError),
    #[error("Error during tokenization: ")]
    Tokenizer(#[from] TokenizerError),
}
