use crate::{
    common::{opcode::OpCode, value::AsValue},
    frontend::ast::{
        expression::Expression, identifier::Identifier, node::AsNode, CompileToBytecode,
    },
};

use super::AsStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub identifier: Identifier,
    pub intializer: Expression,
    pub is_global: bool,
    // pub mutable: bool,
}
impl CompileToBytecode for VariableDeclaration {
    fn to_bytecode(self, function: &mut crate::common::function::Function) -> () {
        assert!(self.is_global);
        if self.is_global {
            self.intializer.to_bytecode(function);
            let name = function.chunk.emit_value(self.identifier.name.as_value());
            function.chunk.emit_op(OpCode::SetGlobal(name))
        }
    }
}
impl AsStatement for VariableDeclaration {
    fn as_statement(self) -> super::Statement {
        super::Statement::VariableDeclaration(self)
    }
}
impl AsNode for VariableDeclaration {
    fn as_node(self) -> crate::frontend::ast::node::Node {
        self.as_statement().as_node()
    }
}
