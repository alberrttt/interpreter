use crate::{
    common::{opcode::OpCode, value::AsValue},
    frontend::{
        compiler::Compiler,
        scanner::{Token, TokenKind},
    },
};

use super::{
    node::{AsNode, Node},
    CompileToBytecode,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub value: Token,
}

impl CompileToBytecode for Identifier {
    fn to_bytecode(self, compiler: &mut Compiler) {
        let local = compiler.resolve_local(&self.value);
        let function = &mut compiler.function;
        #[allow(unused_assignments)]
        let mut op: OpCode = OpCode::Nop;
        if let Some(index) = local {
            op = OpCode::GetLocal(index as u16);
        } else {
            let name = function.chunk.emit_value(self.value.lexeme.to_value());
            op = OpCode::GetGlobal(name)
        }

        function.chunk.emit_op(op);
    }
}
impl<'a> Compiler<'a> {
    pub fn resolve_local(&mut self, name: &Token) -> Option<usize> {
        assert_eq!(name.kind, TokenKind::Identifier);
        for (i, token) in self.locals[0..self.local_count].iter().enumerate().rev() {
            if name.lexeme.eq(&token.name.lexeme) {
                return Some(i);
            }
        }

        None
    }
}
impl AsNode for Identifier {
    fn to_node(self) -> super::node::Node {
        Node::Identifier(self)
    }
}
