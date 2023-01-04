use crate::frontend::scanner::{Position, Token, TokenKind};

#[derive(Debug, Default, Clone)]
pub struct Local {
    pub name: Token,
    pub depth: u8,
}
impl Local {
    pub fn new() -> Local {
        Local {
            name: Token::default(),
            depth: 0,
        }
    }
}
pub const LOCAL: Local = Local {
    name: Token {
        kind: TokenKind::Error,
        lexeme: String::new(),
        line: 9999,
        length: 9999,
        position: Position {
            start_in_line: 9999,
            start_in_source: 9999,
            line: 9999,
        },
    },
    depth: 0,
};
