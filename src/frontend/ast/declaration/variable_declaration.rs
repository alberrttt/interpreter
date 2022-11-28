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
    },
};

use super::AsDeclaration;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub identifier: Identifier,
    pub intializer: Expression,
    pub is_global: bool,
    // pub mutable: bool,
}
impl CompileToBytecode for VariableDeclaration {
    fn to_bytecode(self, compiler: &mut Compiler) -> () {
        assert!(self.is_global);
        if !self.is_global {
            return;
        }
        self.intializer.to_bytecode(compiler);
        let function = &mut compiler.function;
        let name = function
            .chunk
            .emit_value(self.identifier.name.as_value().clone());
        function.chunk.emit_op(OpCode::DefineGlobal(name))
    }
}

impl AsDeclaration for VariableDeclaration {
    fn as_declaration(self) -> super::Declaration {
        super::Declaration::VariableDeclaration(self)
    }
}
impl AsNode for VariableDeclaration {
    fn as_node(self) -> crate::frontend::ast::node::Node {
        Node::Declaration(self.as_declaration())
    }
}
