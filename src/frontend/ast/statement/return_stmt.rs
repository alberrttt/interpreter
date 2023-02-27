use crate::{
    common::opcode::OpCode,
    frontend::{
        ast::{
            expression::Expression,
            literal::{Literal, Literals},
            CompileToBytecode,
        },
        scanner::Token,
    },
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
                let this = Literal(
                    Literals::Void,
                    Token {
                        ..Default::default()
                    },
                );
                &Expression::Literal(this)
            })
            .to_bytecode(compiler);
        compiler.bytecode.function.chunk.emit_op(OpCode::Return);
        compiler.bytecode.returned = true;
    }
}
