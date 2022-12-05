use std::{char, fmt};

use super::ast::expression::comparison::{Comparison, ComparisonKind};

#[derive(Debug, Clone, Default)]
pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub tokens: Vec<Token>,
    pub line_info: LineInfo,
}
macro_rules! token {
    ($self:ident, Error, $reason:expr) => {{
        Token {
            kind: TokenKind::Error,
            lexeme: $reason,
            line: $self.line,
            length: $self.current - $self.start,
            position: Position {
                line: $self.line,
                start_in_line: $self.line_info.start,
                start_in_source: ($self.start as u16),
            },
        }
    }};
    ($self:ident, Identifier) => {{
        Token {
            kind: $self.to_identifier(),
            lexeme: $self.source[$self.start..$self.current].to_string(),
            line: $self.line,
            length: $self.current - $self.start,
            position: Position {
                line: $self.line,
                start_in_line: $self.line_info.start,
                start_in_source: ($self.start as u16),
            },
        }
    }};
    ($self:ident, $kind:ident) => {{
        Token {
            kind: TokenKind::$kind,
            lexeme: $self.source[$self.start..$self.current].to_string(),
            line: $self.line,
            length: $self.current - $self.start,
            position: Position {
                line: $self.line,
                start_in_line: $self.line_info.start,
                start_in_source: ($self.start as u16),
            },
        }
    }};
}
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub length: usize,
    pub position: Position,
}
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Position {
    pub line: usize,
    pub start_in_line: u16,
    pub start_in_source: u16,
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value: {} Kind: {}", self.lexeme, self.kind)
    }
}
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum TokenKind {
    Identifier,
    Let,
    Mut,
    Use,
    Return,

    AssertEq,
    AssertNe,
    Print,
    And,
    Or,
    If,
    Else,
    Nil,
    While,
    For,
    False,
    True,
    Func,

    RightBrace,
    LeftBrace,
    RightParen,
    LeftParen,

    Equal,
    EqualEqual,

    Number,
    String,

    Plus,
    PlusEqual,
    Dash,
    DashEqual,
    Slash,
    SlashEqual,
    Star,
    StarEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    SemiColon,
    Comma,

    Error,
    #[default]
    WHITESPACE,
    EOF,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineInfo {
    pub current: u16,
    pub start: u16,
}
impl Scanner {
    pub fn scan_thru(&mut self) {
        loop {
            if self.at_end() {
                break;
            }
            self.next();
        }
    }
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 0,
            tokens: Vec::new(),
            line_info: LineInfo {
                current: 0,
                start: 0,
            },
        }
    }
    pub fn reset(&mut self, source: String) {
        self.source = source;
        self.start = 0;
        self.current = 0;
        self.line = 0;
        self.line_info.current = 0;
        self.line_info.start = 0;
        self.tokens.clear()
    }
    pub fn next(&mut self) -> Token {
        self.ignore_whitespace();

        if self.at_end() {
            return token!(self, EOF);
        }
        self.start = self.current;
        self.line_info.start = self.line_info.current;
        let char = self.advance();
        if char.is_alphabetic() {
            let token = self.identifier();
            self.tokens.push(token.clone());
            return token;
        }
        if char.is_ascii_digit() {
            let token = self.number();
            self.tokens.push(token.clone());
            return token;
        }

        let token = match char {
            '"' => self.string(),
            '=' => {
                if self.matches('=') {
                    return token!(self, EqualEqual);
                }
                token!(self, Equal)
            }
            '*' => token!(self, Star),
            '/' => {
                if self.matches('=') {
                    return token!(self, SlashEqual);
                }
                token!(self, Slash)
            }

            '+' => {
                if self.matches('=') {
                    return token!(self, PlusEqual);
                }
                token!(self, Plus)
            }
            '-' => {
                if self.matches('=') {
                    return token!(self, DashEqual);
                }
                token!(self, Dash)
            }
            '>' => {
                if self.matches('=') {
                    return token!(self, GreaterEqual);
                }
                token!(self, Greater)
            }
            '<' => {
                if self.matches('=') {
                    return token!(self, LessEqual);
                }
                token!(self, Less)
            }
            '{' => token!(self, LeftBrace),
            '(' => token!(self, LeftParen),
            ')' => token!(self, RightParen),
            '}' => token!(self, RightBrace),
            ';' => token!(self, SemiColon),
            ',' => token!(self, Comma),
            '!' => {
                if self.matches('=') {
                    return token!(self, BangEqual);
                }
                token!(self, Bang)
            }
            _ => token!(self, Error, "unexpected character".to_string()),
        };
        self.tokens.push(token.clone());
        token
    }
}
impl Scanner {
    fn advance(&mut self) -> char {
        self.current += 1;
        self.line_info.current += 1;
        self.source.as_bytes()[self.current - 1] as char
    }
    fn peek(&self) -> char {
        self.source.as_bytes()[self.current] as char
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            return self.source.as_bytes()[self.current + 1] as char;
        }
    }
    pub fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn remaining(&self) -> usize {
        self.source.len() - self.current
    }
    fn matches(&mut self, to: char) -> bool {
        if self.at_end() || self.peek() != to {
            false
        } else {
            self.current += 1;
            true
        }
    }
    fn lexeme(&self) -> &str {
        &self.source[self.start..self.current]
    }
    fn ignore_whitespace(&mut self) {
        while !self.at_end() {
            let peeked = self.peek();
            match peeked {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    self.line_info.current = 0;
                    self.line_info.start = 0;
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while !self.at_end() && self.peek() != '\n' {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _x => {
                    break;
                }
            }
        }
    }
    fn string(&mut self) -> Token {
        while !self.at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.at_end() {
            return token!(self, Error, String::from("unterminated string"));
        }
        self.advance();
        Token {
            kind: TokenKind::String,
            lexeme: self.source[self.start + 1..self.current - 1].to_string(),
            line: self.line,
            length: self.current - self.start,
            position: Position {
                line: self.line,
                start_in_line: self.line_info.start,
                start_in_source: self.start as u16,
            },
        }
    }
    fn number(&mut self) -> Token {
        while !self.at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.remaining() >= 2 && self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while !self.at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        token!(self, Number)
    }
    fn to_identifier(&mut self) -> TokenKind {
        match self.lexeme() {
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "use" => TokenKind::Use,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "nil" => TokenKind::Nil,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "false" => TokenKind::False,
            "true" => TokenKind::True,
            "func" => TokenKind::Func,
            "return" => TokenKind::Return,
            "print" => TokenKind::Print,
            "assert_eq" => TokenKind::AssertEq,
            "assert_ne" => TokenKind::AssertNe,
            _ => TokenKind::Identifier,
        }
    }
    fn identifier(&mut self) -> Token {
        while !self.at_end() && {
            let char = self.peek();
            char.is_alphanumeric() || char == '_'
        } {
            self.advance();
        }
        let tkn = token!(self, Identifier);
        tkn
    }
}

impl TryFrom<TokenKind> for ComparisonKind {
    type Error = String;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            TokenKind::Greater => Ok(ComparisonKind::Greater),
            TokenKind::GreaterEqual => Ok(ComparisonKind::GreaterEq),
            TokenKind::Less => Ok(ComparisonKind::Less),
            TokenKind::LessEqual => Ok(ComparisonKind::LessEq),
            x => Err(format!("cannot convert {} to comparison kind", x)),
        }
    }
}
