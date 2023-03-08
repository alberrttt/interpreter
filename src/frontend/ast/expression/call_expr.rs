use core::panic;

use colored::Colorize;

use crate::{
    backend::vm::natives::MACROS::idx_to_str,
    common::opcode::OpCode,
    frontend::{
        ast::CompileToBytecode,
        declaration::function::Parameter,
        identifier::Identifier,
        literal::Literal,
        types::{Primitive, Signature},
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
        let Signature::Function { params, return_type } = call_sig else {
            panic!()
        };
        self.expr.to_bytecode(compiler);
        self.parameters.iter().enumerate().for_each(|(idx, param)| {
            let param_sig = params[idx].clone();
            let param_type = param.clone();
            let param_type = match param_type {
                Expression::Literal(lit) => {
                    let type_of: Primitive = lit.into();
                    type_of
                }
                Expression::Identifier(ident) => {
                    let type_of = compiler
                        .bytecode
                        .scope
                        .last()
                        .unwrap()
                        .get(&ident.value.lexeme)
                        .unwrap()
                        .clone();
                    let Signature::Variable(type_of) = type_of else {
                        panic!()
                    };
                    *type_of
                }
                x => panic!("{x:?}"),
            };
            if param_sig != param_type {
                compiler.diagnostics.borrow_mut().log(
                    None,
                    "Type Error",
                    format!(
                        "Expected {expect} but got {got}",
                        expect = format!("{param_sig:?}").yellow(),
                        got = format!("{param_type:?}").bright_red()
                    )
                    .bold()
                    .to_string(),
                );
                panic!()
            }
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
