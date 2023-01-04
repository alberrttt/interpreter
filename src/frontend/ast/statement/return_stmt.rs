use crate::{
    common::opcode::OpCode,
    frontend::ast::{expression::Expression, literal::Literal, CompileToBytecode},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub expr: Option<Expression>,
}

impl CompileToBytecode for ReturnStmt {
    fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        self.expr
            .as_ref()
            .unwrap_or({
                let this = Literal::Void;
                &Expression::Literal(this)
            })
            .to_bytecode(compiler);
        compiler.function.chunk.emit_op(OpCode::Return);
    }
}
