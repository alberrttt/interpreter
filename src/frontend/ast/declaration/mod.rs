use self::variable_declaration::VariableDeclaration;

use super::CompileToBytecode;

pub mod variable_declaration;
#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    VariableDeclaration(VariableDeclaration),
}

impl CompileToBytecode for Declaration {
    fn to_bytecode(self, function: &mut crate::common::function::Function) -> () {
        match self {
            Declaration::VariableDeclaration(declaration) => declaration.to_bytecode(function),
        }
    }
}
pub trait AsDeclaration {
    fn as_declaration(self) -> Declaration;
}
