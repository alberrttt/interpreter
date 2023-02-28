use crate::{
    common::{opcode::OpCode, value::AsValue},
    frontend::{
        ast::{
            expression::Expression,
            identifier::Identifier,
            node::{AsNode, Node},
            CompileToBytecode,
        },
        compiler::{local::Local, Compiler},
        scanner::Token,
    },
};

use super::AsDeclaration;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub identifier: Identifier,
    pub intializer: Expression,
    // pub mutable: bool,
}
impl CompileToBytecode for VariableDeclaration {
    fn to_bytecode(&self, compiler: &mut Compiler) {
        self.intializer.to_bytecode(compiler);
        if compiler.bytecode.scope_depth > 0 {
            compiler.add_local(self.identifier.value.clone());
            return;
        }
        let function = &mut compiler.bytecode.function;
        let lexeme = self.identifier.value.lexeme.clone();
        let name = function.chunk.emit_value(lexeme.to_value());
        compiler.bytecode.globals.push(lexeme);
        function.chunk.emit_op(OpCode::DefineGlobal(name))
    }
}
impl<'a> Compiler<'a> {
    pub fn add_local(&mut self, name: Token) {
        self.bytecode.locals.push(Local {
            name,
            depth: self.bytecode.scope_depth,
            is_captured: false,
        });
        self.bytecode.local_count += 1;
    }
}
impl AsDeclaration for VariableDeclaration {
    fn to_declaration(self) -> super::Declaration {
        super::Declaration::VariableDeclaration(self)
    }
}
impl AsNode for VariableDeclaration {
    fn to_node(self) -> crate::frontend::ast::node::Node {
        Node::Declaration(self.to_declaration())
    }
}
