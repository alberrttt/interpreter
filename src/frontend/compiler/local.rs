use crate::frontend::scanner::{Position, Token, TokenKind};

#[derive(Debug, Default, Clone)]
pub struct Local {
    pub name: Token,
    pub depth: u8,
    pub is_captured: bool,
}
impl Local {
    pub fn new() -> Local {
        Local {
            name: Token::default(),
            depth: 0,
            is_captured: false,
        }
    }
}
pub const LOCAL: Local = Local {
    name: Token {
        kind: TokenKind::Error,
        lexeme: String::new(),
        line: 1234,
        length: 4321,
        position: Position {
            start_in_line: 1234,
            start_in_source: 4321,
            line: 1234,
        },
    },
    depth: 255,
    is_captured: false,
};
