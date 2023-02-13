use std::ptr::addr_of_mut;

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
        } else if let Some(arg) = {
            let tmp = compiler.resolve_up_value(&self.value);
            tmp
        } {
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
    pub fn add_up_value(&mut self, index: usize, is_local: bool) -> Option<usize> {
        let up_value_count = &mut self.bytecode.function.upvalue_count;
        // cjeck if the upvalue is already in
        for (i, up_value) in self.bytecode.upvalues[0..*up_value_count]
            .iter()
            .enumerate()
        {
            if up_value.index == index as u8 && up_value.is_local == is_local {
                dbg!(up_value);

                return Some(i);
            }
        }

        if *up_value_count == 255 {
            panic!("Too many upvalues");
        }

        // WORK HERE
        self.bytecode.upvalues[*up_value_count].is_local = is_local;
        self.bytecode.upvalues[*up_value_count].index = index as u8;

        {
            *up_value_count += 1;
            Some(*up_value_count - 1)
        }
    }
    pub fn resolve_up_value(&mut self, token: &Token) -> Option<usize> {
        let compiler = self.enclosing.as_mut()?.get_compiler();
        let local = compiler.resolve_local(token);

        if let Some(local) = local {
            dbg!(local);
            return self.add_up_value(local, true);
        }

        let upvalue = compiler.resolve_up_value(token);
        if let Some(upvalue) = upvalue {
            dbg!(upvalue);
            return self.add_up_value(upvalue, false);
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
