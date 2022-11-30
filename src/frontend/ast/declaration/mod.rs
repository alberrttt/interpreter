use crate::frontend::compiler::Compiler;

use self::variable_declaration::VariableDeclaration;

use super::CompileToBytecode;

pub mod variable_declaration;
#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    VariableDeclaration(VariableDeclaration),
}

impl CompileToBytecode for Declaration {
    fn to_bytecode(self, compiler: &mut Compiler) -> () {
        let Compiler {
            function: _,
            scanner: _,
            parser: _,
            context: _,
        } = compiler;
        match self {
            Declaration::VariableDeclaration(declaration) => declaration.to_bytecode(compiler),
        }
    }
}
pub trait AsDeclaration {
    fn as_declaration(self) -> Declaration;
}
