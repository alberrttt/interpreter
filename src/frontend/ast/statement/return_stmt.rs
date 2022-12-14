use crate::{
    common::{function, opcode::OpCode},
    frontend::ast::{
        expression::{AsExpr, Expression},
        literal::Literal,
        CompileToBytecode,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub expr: Option<Expression>,
}

impl CompileToBytecode for ReturnStmt {
    fn to_bytecode(self, compiler: &mut crate::frontend::compiler::Compiler) -> () {
        let _diagnostics = &mut compiler.context.as_mut().unwrap().diagnostics;
        self.expr
            .unwrap_or(Literal::Void.as_expr())
            .to_bytecode(compiler);
        compiler.function.chunk.emit_op(OpCode::Return);
    }
}
