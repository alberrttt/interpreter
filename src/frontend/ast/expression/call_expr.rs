use crate::{
    common::opcode::OpCode,
    frontend::ast::{identifier::Identifier, CompileToBytecode},
};

use super::AsExpr;
#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub identifier: Identifier,
}
impl CompileToBytecode for CallExpr {
    fn to_bytecode(self, compiler: &mut crate::frontend::compiler::Compiler) -> () {
        self.identifier.to_bytecode(compiler);
        compiler.function.chunk.emit_op(OpCode::Call);
    }
}
impl AsExpr for CallExpr {
    fn as_expr(self) -> super::Expression {
        super::Expression::CallExpr(self)
    }
}
