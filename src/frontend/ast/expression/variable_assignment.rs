use crate::{
    common::{opcode::OpCode, value::AsValue},
    frontend::ast::{identifier::Identifier, CompileToBytecode},
};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssignment {
    pub initializer: Box<Expression>,
    pub name: Identifier,
    pub global: bool,
}
impl CompileToBytecode for VariableAssignment {
    fn to_bytecode(self, compiler: &mut crate::frontend::compiler::Compiler) -> () {
        if !self.global {
            todo!()
        }
        self.initializer.to_bytecode(compiler);
        let name = compiler
            .function
            .chunk
            .emit_value(self.name.name.value.as_value());
        compiler.function.chunk.emit_op(OpCode::SetGlobal(name))
    }
}
