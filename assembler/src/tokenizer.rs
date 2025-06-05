use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Token<'a> {
    Label(String),
    Register(String),
    Literal(Literal<'a>),
    Instruction(String),
    Comma,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Literal<'a> {
    Decimal(&'a str),
    Binary(&'a str),
    Hexadecimal(&'a str),
    Octal(&'a str),
    Boolean(bool),
    String(&'a str),
    Char(char),
}

pub(crate) struct Tokenizer<'a> {
    tokens: Vec<Token<'a>>,
    curr_idx: usize,
    token_start_idx: usize,
    input: &'a str,
    input_len: usize,
    errors: Option<Vec<TokenizerError>>,
}

impl Tokenizer<'_> {
    const fn from(input: &str) -> Tokenizer {
        Tokenizer {
            tokens: Vec::new(),
            curr_idx: 0,
            token_start_idx: 0,
            input,
            input_len: input.len(),
            errors: None,
        }
    }

    pub(crate) fn tokenize(input: &str) -> Result<Vec<Token>, Vec<TokenizerError>> {
        let mut tokenizer = Tokenizer::from(input);

        tokenizer.run();

        match tokenizer.errors {
            Some(errors) => Err(errors),
            None => Ok(tokenizer.tokens),
        }
    }

    fn run(&mut self) {
        while self.curr_idx < self.input_len {
            self.token_start_idx = self.curr_idx;

            match self.get_curr_char() {
                '.' => self.expect_label(),
                'R' => self.expect_register(),
                '#' => self.expect_literal(),
                ',' => self.expect_comma(),
                c if c.is_alphabetic() => self.expect_instruction(),
                c if c.is_whitespace() => self.curr_idx += 1,
                c => {
                    self.curr_idx += 1;
                    self.add_error(TokenizerError::InvalidTokenStart {
                        start: c,
                        idx: self.curr_idx,
                    });
                }
            }
        }
    }

    #[inline]
    fn add_error(&mut self, err: TokenizerError) {
        self.errors.get_or_insert_default().push(err);
    }

    fn get_curr_char(&self) -> char {
        self.input.chars().nth(self.curr_idx).map_or_else(||unreachable!(
            "The index should not be greater or equal to the length of the input. This should never happen."
        ), |c|c.to_uppercase().next().expect("Not a valid character."))
    }

    fn set_curr_idx_to_token_end(&mut self) {
        if self.get_curr_char().is_whitespace() {
            return;
        }

        while self.curr_idx < self.input_len && !self.get_curr_char().is_whitespace() {
            self.curr_idx += 1;
        }

        self.curr_idx -= 1;
    }

    fn expect_label(&mut self) {
        self.curr_idx += 1;

        while self.curr_idx < self.input_len && self.get_curr_char().is_alphabetic() {
            self.curr_idx += 1;
        }

        self.tokens.push(Token::Label(
            self.input[self.token_start_idx..self.curr_idx].to_uppercase(),
        ));
    }

    fn expect_instruction(&mut self) {
        self.curr_idx += 1;

        while self.curr_idx < self.input_len && self.get_curr_char().is_alphabetic() {
            self.curr_idx += 1;
        }

        let inst = self.input[self.token_start_idx..self.curr_idx].to_uppercase();

        let token = if inst == "END" {
            Token::End
        } else {
            Token::Instruction(inst)
        };

        self.tokens.push(token);
    }

    fn expect_register(&mut self) {
        self.curr_idx += 1;

        while self.curr_idx < self.input_len && self.get_curr_char().is_numeric() {
            self.curr_idx += 1;
        }

        self.tokens.push(Token::Register(
            self.input[self.token_start_idx..self.curr_idx].to_uppercase(),
        ));
    }

    fn expect_comma(&mut self) {
        self.tokens.push(Token::Comma);
        self.curr_idx += 1;
    }

    fn expect_literal(&mut self) {
        self.curr_idx += 1;

        match self.get_curr_char() {
            '\'' => self.expect_char_literal(),
            '"' => self.expect_string_literal(),
            c if c.is_numeric() => self.expect_numeric_literal(),
            'T' => self.expect_boolean_true_literal(),
            'F' => self.expect_boolean_false_literal(),
            _ => self.add_error(TokenizerError::InvalidLiteral { idx: self.curr_idx }),
        }

        self.curr_idx += 1;
    }

    fn expect_char_literal(&mut self) {
        self.curr_idx += 1;

        let c = self.get_curr_char();

        self.curr_idx += 1;

        match self.get_curr_char() {
            '\'' => self.tokens.push(Token::Literal(Literal::Char(c))),
            _ => self.add_error(TokenizerError::InvalidCharLiteral { idx: self.curr_idx }),
        }
    }

    fn expect_string_literal(&mut self) {
        self.curr_idx += 1;

        while self.get_curr_char() != '"' {
            self.curr_idx += 1;
        }

        // +2 to ignore the prefix #"
        self.tokens.push(Token::Literal(Literal::String(
            &self.input[self.token_start_idx + 2..self.curr_idx],
        )));
    }

    fn expect_numeric_literal(&mut self) {
        let literal = if self.get_curr_char() == '0' {
            self.curr_idx += 1;
            self.token_start_idx = self.curr_idx;
            match self.get_curr_char() {
                'B' => {
                    self.set_curr_idx_to_token_end();
                    Literal::Binary(&self.input[self.token_start_idx + 1..=self.curr_idx])
                }
                'X' => {
                    self.set_curr_idx_to_token_end();
                    Literal::Hexadecimal(&self.input[self.token_start_idx + 1..=self.curr_idx])
                }
                'O' => {
                    self.set_curr_idx_to_token_end();
                    Literal::Octal(&self.input[self.token_start_idx + 1..=self.curr_idx])
                }
                'D' => {
                    self.set_curr_idx_to_token_end();
                    Literal::Decimal(&self.input[self.token_start_idx + 1..=self.curr_idx])
                }
                _ => {
                    self.set_curr_idx_to_token_end();
                    Literal::Decimal(&self.input[self.token_start_idx..=self.curr_idx])
                }
            }
        } else {
            self.set_curr_idx_to_token_end();
            Literal::Decimal(&self.input[self.token_start_idx + 1..=self.curr_idx])
        };

        self.tokens.push(Token::Literal(literal));
    }

    fn expect_boolean_true_literal(&mut self) {
        self.curr_idx += 4; // len of "true"

        // +1 to ignore prefix #
        match self.input[self.token_start_idx + 1..self.curr_idx]
            .to_uppercase()
            .as_str()
        {
            "TRUE" => self.tokens.push(Token::Literal(Literal::Boolean(true))),
            _ => self.add_error(TokenizerError::InvalidBooleanTrueLiteral {
                idx: self.token_start_idx,
            }),
        }
    }

    fn expect_boolean_false_literal(&mut self) {
        self.curr_idx += 5; // len of "false"

        // +1 to ignore prefix #
        match self.input[self.token_start_idx + 1..self.curr_idx]
            .to_uppercase()
            .as_str()
        {
            "FALSE" => self.tokens.push(Token::Literal(Literal::Boolean(false))),
            _ => self.add_error(TokenizerError::InvalidBooleanFalseLiteral {
                idx: self.token_start_idx,
            }),
        }
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum TokenizerError {
    #[error("Token at idx {idx} is not allowed to start with {start}. ")]
    InvalidTokenStart { start: char, idx: usize },
    #[error("Invalid literal at idx: {idx}.")]
    InvalidLiteral { idx: usize },
    #[error("Expected char literal at idx {idx} to end with \'.")]
    InvalidCharLiteral { idx: usize },
    #[error("Expected boolean literal TRUE/true at idx {idx}.")]
    InvalidBooleanTrueLiteral { idx: usize },
    #[error("Expected boolean literal FALSE/false at idx {idx}.")]
    InvalidBooleanFalseLiteral { idx: usize },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {}

    #[test]
    fn test_run() {
        let mut t = Tokenizer::from(
            "
            .main
                MOV R0, #5
                nop
                MOV R256, #0xBc2a
                Mul R0, r256
                JMP .main
            ",
        );
        t.run();
        assert_eq!(
            t.tokens,
            vec![
                Token::Label(".MAIN".into()),
                Token::Instruction("MOV".into()),
                Token::Register("R0".into()),
                Token::Comma,
                Token::Literal(Literal::Decimal("5")),
                Token::Instruction("NOP".into()),
                Token::Instruction("MOV".into()),
                Token::Register("R256".into()),
                Token::Comma,
                Token::Literal(Literal::Hexadecimal("Bc2a".into())),
                Token::Instruction("MUL".into()),
                Token::Register("R0".into()),
                Token::Comma,
                Token::Register("R256".into()),
                Token::Instruction("JMP".into()),
                Token::Label(".MAIN".into())
            ]
        );
    }

    #[test]
    fn test_add_error() {
        let mut t = Tokenizer::from("");
        let err = TokenizerError::InvalidTokenStart { start: ' ', idx: 0 };
        assert!(t.errors.is_none());
        t.add_error(err.clone());
        assert_eq!(t.errors.unwrap(), vec![err.into()]);
    }

    #[test]
    fn test_get_curr_char() {
        let t = Tokenizer::from(".main mov");
        assert_eq!(t.get_curr_char(), '.');
    }

    #[test]
    #[should_panic]
    fn test_get_curr_char_out_of_bounds() {
        let mut t = Tokenizer::from(".main");
        assert_eq!(t.get_curr_char(), '.');
        t.curr_idx += 5;
        let _ = t.get_curr_char(); // panic
    }

    #[test]
    fn test_expect_label() {
        let mut t = Tokenizer::from(".main");
        t.expect_label();
        assert_eq!(t.tokens[0], Token::Label(".MAIN".into()));
        t = Tokenizer::from(".MAIN");
        t.expect_label();
        assert_eq!(t.tokens[0], Token::Label(".MAIN".into()))
    }

    #[test]
    fn test_expect_instruction() {
        let mut t = Tokenizer::from("mov");
        t.expect_instruction();
        assert_eq!(t.tokens[0], Token::Instruction("MOV".into()));
        t = Tokenizer::from("JMP");
        t.expect_instruction();
        assert_eq!(t.tokens[0], Token::Instruction("JMP".into()));
    }

    #[test]
    fn test_expect_register() {
        let mut t = Tokenizer::from("R0");
        t.expect_register();
        assert_eq!(t.tokens[0], Token::Register("R0".into()));
        t = Tokenizer::from("R4242");
        t.expect_register();
        assert_eq!(t.tokens[0], Token::Register("R4242".into()));
    }

    #[test]
    fn test_expect_comma() {
        let mut t = Tokenizer::from(",");
        t.expect_comma();
        assert_eq!(t.tokens[0], Token::Comma);
    }

    #[test]
    fn test_expect_literal() {
        let mut t = Tokenizer::from("#42");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Decimal("42")));
        let mut t = Tokenizer::from("#0x4H");
        t.expect_literal();
        assert_eq!(
            t.tokens[0],
            Token::Literal(Literal::Hexadecimal("4H".into()))
        );
        let mut t = Tokenizer::from("#0b010110");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Binary("010110")));
        let mut t = Tokenizer::from("#0o743");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Octal("743")));
        let mut t = Tokenizer::from("#true");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Boolean(true)));
        let mut t = Tokenizer::from("#false");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Boolean(false)));
        let mut t = Tokenizer::from("#\"Hello, there\"");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::String("Hello, there")));
        let mut t = Tokenizer::from("#\'7\'");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Char('7')));
    }

    #[test]
    fn test_expect_char_literal() {
        let mut t = Tokenizer::from("#\'B\'");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Char('B')));
    }

    #[test]
    fn test_expect_string_literal() {
        let mut t = Tokenizer::from("#\"Jajajajaja2498291849102+#amfl929r2jlsamfa3\"");
        t.expect_literal();
        assert_eq!(
            t.tokens[0],
            Token::Literal(Literal::String(
                "Jajajajaja2498291849102+#amfl929r2jlsamfa3"
            ))
        );
    }

    #[test]
    fn test_expect_numeric_literal() {
        let mut t = Tokenizer::from("#42");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Decimal("42")));
        let mut t = Tokenizer::from("#0d42");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Decimal("42")));
        let mut t = Tokenizer::from("#0x4H");
        t.expect_literal();
        assert_eq!(
            t.tokens[0],
            Token::Literal(Literal::Hexadecimal("4H".into()))
        );
        let mut t = Tokenizer::from("#0b010110");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Binary("010110")));
        let mut t = Tokenizer::from("#0o743");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Octal("743")));
    }

    #[test]
    fn test_expect_boolean_true_literal() {
        let mut t = Tokenizer::from("#TRUE");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Boolean(true)));
    }

    #[test]
    fn test_expect_boolean_false_literal() {
        let mut t = Tokenizer::from("#FALSE");
        t.expect_literal();
        assert_eq!(t.tokens[0], Token::Literal(Literal::Boolean(false)));
    }
}
