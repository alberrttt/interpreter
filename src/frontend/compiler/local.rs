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
