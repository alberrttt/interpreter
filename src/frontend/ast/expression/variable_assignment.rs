use crate::{
    common::{opcode::OpCode, value::AsValue},
    frontend::ast::{identifier::Identifier, CompileToBytecode},
};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssignment {
    pub initializer: Box<Expression>,
    pub name: Identifier,
}
impl CompileToBytecode for VariableAssignment {
    fn to_bytecode(self, compiler: &mut crate::frontend::compiler::Compiler) -> () {
        self.initializer.to_bytecode(compiler);
        let local = compiler.resolve_local(&self.name.value);
        if let Some(local) = local {
            compiler
                .function
                .chunk
                .emit_op(OpCode::SetLocal(local as u16));
            return;
        }
        let name = compiler
            .function
            .chunk
            .emit_value(self.name.value.lexeme.as_value());
        compiler.function.chunk.emit_op(OpCode::SetGlobal(name))
    }
}
