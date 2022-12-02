use crate::{
    common::{
        opcode::OpCode,
        value::{AsValue},
    },
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
    pub is_global: bool,
    // pub mutable: bool,
}
impl CompileToBytecode for VariableDeclaration {
    fn to_bytecode(self, compiler: &mut Compiler) -> () {
        self.intializer.to_bytecode(compiler);
        if !self.is_global {
            let OpCode::Constant(index) = compiler.function.chunk.code.pop().unwrap() else {
                panic!()
            };
            compiler.function.chunk.emit_op(OpCode::DefineLocal(index));
            compiler.add_local(self.identifier.name);
            return;
        }
        let function = &mut compiler.function;
        let name = function
            .chunk
            .emit_value(self.identifier.name.value.as_value().clone());
        function.chunk.emit_op(OpCode::DefineGlobal(name))
    }
}
impl<'a> Compiler<'a> {
    pub fn add_local(&mut self, name: Token) {
        let local = &mut self.locals[self.local_count];
        self.local_count += 1;

        local.name = name;
        local.depth = self.scope_depth;
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
