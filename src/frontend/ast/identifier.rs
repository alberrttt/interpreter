use colored::Colorize;

use crate::{
    common::{opcode::OpCode, value::AsValue},
    frontend::{
        bytecode::scope,
        compiler::Compiler,
        scanner::{Token, TokenKind},
        typesystem::ResolveSignature,
    },
};

use super::{
    super::typesystem::{Primitive, Signature},
    expression::Expression,
    node::{AsNode, Node},
    CompileToBytecode,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub value: Token,
}
impl From<Identifier> for Expression {
    fn from(ident: Identifier) -> Self {
        Expression::Identifier(ident)
    }
}
impl From<Expression> for Identifier {
    fn from(expr: Expression) -> Self {
        match expr {
            Expression::Identifier(ident) => ident,
            _ => panic!(),
        }
    }
}
impl ResolveSignature for Identifier {
    fn resolve_signature(&self, compiler: &mut Compiler) -> Signature {
        // check if the surrounding scope has a signature
        // if it does, then return that signature
        for scope_depth in 0..(compiler.bytecode.scope_depth + 1) {
            let scope = compiler.bytecode.scope.get(scope_depth).unwrap();
            if let Some(signature) = scope.get(&self.value.lexeme) {
                return signature.clone();
            }
        }

        // if the surrounding scope does not have a signature, that means that the identifier is refering to a non-existant variable
        compiler.diagnostics.borrow_mut().log(
            Some(&self.value.position),
            "Compiler",
            format!("Unable to find variable '{}' \n", self.value.lexeme)
                .bright_red()
                .to_string(),
        );
        panic!()
    }
}
impl CompileToBytecode for Identifier {
    fn to_bytecode(&self, compiler: &mut Compiler) {
        let local = compiler.resolve_local(&self.value);
        #[allow(unused_assignments)]
        let mut op: OpCode = OpCode::Nop;
        if let Some(arg) = local {
            op = OpCode::GetLocal(arg as u16);
        } else if let Some(arg) = { compiler.resolve_up_value(&self.value) } {
            op = OpCode::GetUpValue(arg as u16);
        } else {
            let function = &mut compiler.bytecode.function;
            let lexeme = self.value.lexeme.clone();
            if compiler.bytecode.globals.contains(&lexeme) {
                let arg = function.chunk.emit_value(lexeme.to_value());
                op = OpCode::GetGlobal(arg)
            } else {
                compiler.diagnostics.borrow_mut().log(
                    Some(&self.value.position),
                    "Compiler",
                    format!("Unable to find variable '{}' \n", self.value.lexeme)
                        .bright_red()
                        .to_string(),
                )
            }
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
        let enclosing = self.enclosing.as_mut()?.get_compiler();
        let local = enclosing.resolve_local(token);

        if let Some(index) = local {
            let local = &mut enclosing.bytecode.locals[index];
            local.is_captured = true;
            return self.add_up_value(index, true);
        }

        let upvalue = enclosing.resolve_up_value(token);
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
