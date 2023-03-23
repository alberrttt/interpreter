use core::panic;

use colored::Colorize;

use crate::{
    backend::vm::natives::MACROS::idx_to_str,
    common::opcode::OpCode,
    frontend::{
        ast::CompileToBytecode,
        compiler::Compiler,
        declaration::function::Parameter,
        identifier::Identifier,
        literal::Literal,
        typesystem::{FunctionSignature, Primitive, ResolveSignature, Signature},
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
        let Signature::Function(FunctionSignature{ params, return_type }) = call_sig else {
            panic!()
        };
        self.expr.to_bytecode(compiler);
        self.compile_parameters(compiler);
        compiler
            .bytecode
            .function
            .chunk
            .emit_op(OpCode::Call(self.parameters.len()));
    }
}

impl Call {
    pub fn resolve_function_definition(&self, compiler: &mut Compiler) -> FunctionSignature {
        let name_identifier: Identifier = (*self.expr.to_owned()).into();
        for scope_depth in 0..(compiler.bytecode.scope_depth + 1) {
            let scope = compiler.bytecode.scope.get(scope_depth).unwrap();
            if let Some(signature) = scope.get(&name_identifier.value.lexeme) {
                return if let Signature::Function(func_sig) = signature.clone() {
                    func_sig
                } else {
                    compiler.diagnostics.borrow_mut().log(
                        None,
                        "Type Error",
                        format!(
                            "Expected {expect} but got {got}",
                            expect = format!(
                                "{:?}",
                                Signature::Function(FunctionSignature {
                                    params: vec![],
                                    return_type: Box::new(Primitive::Void)
                                })
                            )
                            .yellow(),
                            got = format!("{:?}", signature).bright_red()
                        )
                        .bold()
                        .to_string(),
                    );
                    panic!()
                };
            }
        }
        compiler.diagnostics.borrow_mut().log(
            None,
            "Type Error",
            format!(
                "Function {function} is not defined",
                function = format!("{}", { name_identifier.value.lexeme }).yellow()
            )
            .bold()
            .to_string(),
        );
        panic!()
    }
    pub fn compile_parameters(&self, compiler: &mut Compiler) {
        let function_definition: FunctionSignature = self.resolve_function_definition(compiler);
        for (i, (argument, declared_param)) in self
            .parameters
            .iter()
            .zip(function_definition.params.iter())
            .enumerate()
        {
            let argument_type: Signature = argument.resolve_signature(compiler);
         
            if argument_type.eq(declared_param.type_annotation.as_ref().unwrap()) {
                argument.to_bytecode(compiler);
                return;
            }
            match argument_type {
                Signature::Function(_) => todo!(),
                Signature::Variable(variable_type) => compiler.diagnostics.borrow_mut().log(
                    None,
                    "Mismatched Types",
                    format!(
                        "`{argument}` has type {} but {expect} was expected",
                        variable_type.as_ref().to_string().yellow(),
                        expect = declared_param
                            .type_annotation
                            .as_ref()
                            .unwrap()
                            .to_string()
                            .yellow(),
                        argument = Identifier::from(argument.to_owned()).value.lexeme.bold()
                    )
                    .bright_red()
                    .to_string(),
                ),
                Signature::Primitive(_) => compiler.diagnostics.borrow_mut().log(
                    None,
                    "Mismatched Types",
                    format!(
                        "Expected {expect} but got {got}",
                        expect = format!("{}", declared_param.type_annotation.as_ref().unwrap())
                            .yellow(),
                        got = format!("{argument_type}").yellow()
                    )
                    .bright_red()
                    .to_string(),
                ),
                Signature::Parameter(_) => todo!(),
            };

            panic!()
        }
    }
}

impl AsExpr for Call {
    fn to_expr(self) -> super::Expression {
        super::Expression::CallExpr(self)
    }
}
