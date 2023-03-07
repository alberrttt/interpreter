use core::panic;

use crate::{
    backend::vm::natives::MACROS::idx_to_str,
    common::opcode::OpCode,
    frontend::{
        ast::CompileToBytecode, identifier::Identifier, literal::Literal, types::Primitive,
    },
};

use super::{AsExpr, Expression};
#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub expr: Box<Expression>,
    pub parameters: Box<Vec<Expression>>,
}

// work to do here
impl CompileToBytecode for Call {
    fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        if let Expression::Identifier(expr) = self.expr.as_ref() {
            if "to_str" == expr.value.lexeme.as_str() {
                self.parameters
                    .iter()
                    .for_each(|param| param.clone().to_bytecode(compiler));
                compiler
                    .bytecode
                    .write_call_fn_arg_ptr_op(idx_to_str!() as u8, self.parameters.len() as u8);
                return;
            }
        }
        let Expression::Identifier(ident) = *self.expr.clone() else {
            panic!()
        };
        let call_sig = compiler
            .bytecode
            .scope
            .last()
            .unwrap()
            .get(&ident.value.lexeme)
            .unwrap()
            .clone();
        self.expr.to_bytecode(compiler);
        self.parameters.iter().for_each(|param| {
            let param_type: Primitive = param.clone().into();
            dbg!(&call_sig);
            param.clone().to_bytecode(compiler);
        });
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
