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
impl VariableAssignment {
    pub fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        self.initializer.to_bytecode(compiler);
        let local = compiler.resolve_local(&self.name.value);
        if let Some(local) = local {
            if compiler.bytecode.compiling_statement {
                compiler
                    .bytecode
                    .function
                    .chunk
                    .emit_op(OpCode::SetLocalConsumes(local as u16));
            } else {
                compiler
                    .bytecode
                    .function
                    .chunk
                    .emit_op(OpCode::SetLocal(local as u16));
            }
            return;
        }
        let name = compiler
            .bytecode
            .function
            .chunk
            .emit_value(self.name.value.lexeme.to_value());
        compiler
            .bytecode
            .function
            .chunk
            .emit_op(OpCode::SetGlobal(name))
    }
}
