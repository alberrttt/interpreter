use crate::{
    common::{opcode::OpCode, value::AsValue},
    frontend::{
        bytecode::Upvalue,
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
    fn to_bytecode(&self, compiler: &mut Compiler) {
        let local = compiler.resolve_local(&self.value);
        #[allow(unused_assignments)]
        let mut op: OpCode = OpCode::Nop;
        if let Some(arg) = local {
            op = OpCode::GetLocal(arg as u16);
        } else if let Some(arg) = compiler.resolve_up_value(&self.value) {
            op = OpCode::GetUpValue(arg as u16);
        } else {
            let function = &mut compiler.bytecode.function;
            let arg = function.chunk.emit_value(self.value.lexeme.to_value());
            op = OpCode::GetGlobal(arg)
        }
        let function = &mut compiler.bytecode.function;
        function.chunk.emit_op(op);
    }
}
impl<'a> Compiler<'a> {
    pub fn add_up_value(&mut self, local: usize, is_local: bool) -> Option<usize> {
        let up_value_count = &mut self.bytecode.function.upvalue_count;

        // WORK HERE

        self.bytecode.upvalues[*up_value_count].is_local = is_local;
        self.bytecode.upvalues[*up_value_count].index = local;

        {
            *up_value_count += 1;
            Some(*up_value_count)
        }
    }
    pub fn resolve_up_value(&mut self, token: &Token) -> Option<usize> {
        self.enclosing.as_ref()?;
        let local = self
            .enclosing
            .as_mut()
            .unwrap()
            .get_compiler()
            .resolve_local(token);
        if let Some(local) = local {
            return self.add_up_value(local, true);
        }

        None
    }
    pub fn resolve_local(&mut self, name: &Token) -> Option<usize> {
        assert_eq!(name.kind, TokenKind::Identifier);
        for (i, token) in self.bytecode.locals[0..self.bytecode.local_count]
            .iter()
            .enumerate()
            .rev()
        {
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
