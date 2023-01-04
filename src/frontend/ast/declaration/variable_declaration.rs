use crate::{
    common::{opcode::OpCode, value::AsValue},
    frontend::{
        ast::{
            expression::Expression,
            identifier::Identifier,
            node::{AsNode, Node},
            CompileToBytecode,
        },
        compiler::Compiler,
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
        let name = function
            .chunk
            .emit_value(self.identifier.value.lexeme.to_value());
        function.chunk.emit_op(OpCode::DefineGlobal(name))
    }
}
impl<'a> Compiler<'a> {
    pub fn add_local(&mut self, name: Token) {
        let local = &mut self.bytecode.locals[self.bytecode.local_count];
        self.bytecode.local_count += 1;

        local.name = name;
        local.depth = self.bytecode.scope_depth;
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
