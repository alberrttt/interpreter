use crate::{
    common::opcode::OpCode,
    frontend::ast::{identifier::Identifier, CompileToBytecode},
};

use super::{AsExpr, Expression};
#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub identifier: Identifier,
    pub parameters: Box<Vec<Expression>>,
}
impl CompileToBytecode for Call {
    fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        // if let "to_str" = self.identifier.value.lexeme.as_str() {
        //     self.parameters
        //         .iter()
        //         .for_each(|param| param.clone().to_bytecode(compiler));
        //     compiler
        //         .bytecode
        //         .write_call_fn_arg_ptr_op(idx_to_str!(), field1);
        //     return;
        // }

        self.identifier.to_bytecode(compiler);
        self.parameters
            .iter()
            .for_each(|param| param.clone().to_bytecode(compiler));
        compiler
            .bytecode
            .function
            .chunk
            .emit_op(OpCode::Call(self.parameters.len()));
    }
}
impl AsExpr for Call {
    fn to_expr(self) -> super::Expression {
        super::Expression::CallExpr(self)
    }
}
