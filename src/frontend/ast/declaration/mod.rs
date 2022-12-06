use crate::frontend::compiler::Compiler;

use self::{function::FunctionDeclaration, variable_declaration::VariableDeclaration};

use super::{CompileToBytecode, node::AsNode};

pub mod function;
pub mod variable_declaration;
#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
}

impl CompileToBytecode for Declaration {
    fn to_bytecode(self, compiler: &mut Compiler) -> () {
        let _function = &mut compiler.function;
        match self {
            Declaration::VariableDeclaration(declaration) => declaration.to_bytecode(compiler),
            Declaration::FunctionDeclaration(function_declaration) => function_declaration.to_bytecode(compiler),
        }
    }
}
pub trait AsDeclaration {
    fn as_declaration(self) -> Declaration;
}
impl AsNode for Declaration {
    fn as_node(self) -> super::node::Node {
        super::node::Node::Declaration(self)
    }
}
