use crate::{
    common::{opcode::OpCode, value::AsValue},
    frontend::{compiler::Compiler, scanner::Token},
};

use super::{
    node::{AsNode, Node},
    CompileToBytecode,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: Token,
}

impl CompileToBytecode for Identifier {
    fn to_bytecode(self, compiler: &mut Compiler) -> () {
        let local = compiler.resolve_local(&self.name);
        let function = &mut compiler.function;
        #[allow(unused_assignments)]
        let mut op: OpCode = OpCode::Nop;
        if let Some(index) = local {
            op = OpCode::GetLocal(index as u16);
        } else {
            let name = function.chunk.emit_value(self.name.value.as_value());
            op = OpCode::GetGlobal(name)
        }

        function.chunk.emit_op(op);
    }
}
impl<'a> Compiler<'a> {
    pub fn resolve_local(&mut self, name: &Token) -> Option<usize> {
        for (i, token) in self.locals[0..self.local_count].iter().enumerate().rev() {
            if name.value.eq(&token.name.value) {
                return Some(i);
            }
        }

        None
    }
}
impl AsNode for Identifier {
    fn as_node(self) -> super::node::Node {
        Node::Identifier(self)
    }
}
