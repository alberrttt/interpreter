use colored::Colorize;

use crate::{
    common::opcode::OpCode,
    frontend::{
        ast::{expression::Expression, CompileToBytecode},
        compiler::FunctionType,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub expr: Option<Expression>,
}

impl CompileToBytecode for ReturnStmt {
    fn to_bytecode(self, compiler: &mut crate::frontend::compiler::Compiler) -> () {
        let diagnostics = &mut compiler.context.as_mut().unwrap().diagnostics;

        if let Some(expr) = self.expr {
            expr.to_bytecode(compiler);
            compiler.function.chunk.emit_op(OpCode::SetTempSlot(0));
            compiler.emit_after_block.push(OpCode::TakeTempSlot(0))
        } else {
        }
        // circumvents the popping operations
    }
}
