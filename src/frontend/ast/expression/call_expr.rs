use crate::{
    common::opcode::OpCode,
    frontend::ast::{identifier::Identifier, CompileToBytecode},
};

use super::{AsExpr, Expression};
#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub identifier: Identifier,
    pub parameters: Box<Vec<Expression>>,
}
impl CompileToBytecode for CallExpr {
    fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        self.identifier.to_bytecode(compiler);
        self.parameters
            .iter()
            .for_each(|param| param.clone().to_bytecode(compiler));
        compiler
            .function
            .chunk
            .emit_op(OpCode::Call(self.parameters.len()));
    }
}
impl AsExpr for CallExpr {
    fn to_expr(self) -> super::Expression {
        super::Expression::CallExpr(self)
    }
}
