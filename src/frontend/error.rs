use std::{
    fmt::{Display},
};

use super::scanner::{Token, TokenKind};
pub type ParseResult<T> = Result<T, ParseError>;
#[derive(Debug, Clone)]
pub enum ParseError {
    SyntaxError(SyntaxError),
    UnexpectedToken {
        expected: TokenKind,
        unexpected: Token,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::SyntaxError(syntax_error) => write!(f, "{syntax_error}"),
            ParseError::UnexpectedToken {
                expected,
                unexpected,
            } => write!(f, "Expected {expected:?} but got {unexpected} instead"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxError(Vec<Token>, String);
impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (tokens, message) = (&self.0, &self.1);
        let mut string = String::new();
        for ele in tokens.iter() {
            match ele.kind {
                TokenKind::Equal => string.push_str(" = "),
                TokenKind::LeftBrace => string.push_str(" {"),
                _ => string.push_str(ele.lexeme.as_ref()),
            }
        }
        write!(f, "{message} occurs here\n {string}")
    }
}
