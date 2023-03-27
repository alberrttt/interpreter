use colored::Colorize;

use crate::{
    common::opcode::OpCode,
    frontend::{
        ast::{
            expression::Expression,
            literal::{Literal, Literals},
            CompileToBytecode,
        },
        scanner::Token,
        typesystem::{Primitive, ResolveSignature, Signature},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub expr: Option<Expression>,
}

impl CompileToBytecode for ReturnStmt {
    fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        let return_value = self.expr.clone().unwrap_or({
            let this = Literal(
                Literals::Void,
                Token {
                    ..Default::default()
                },
            );
            Expression::Literal(this)
        });
        let return_type = match return_value.clone() {
            Expression::Identifier(identifier) => identifier.resolve_signature(compiler).into(),
            Expression::Literal(literal) => Primitive::from(literal),
            _ => todo!(
                "{}",
                format!("unable to resolve the type of `{return_value:?}`").yellow()
            ),
        };
        if (compiler.bytecode.return_type.clone())
            .unwrap_or(Primitive::Void)
            .ne(&return_type)
        {
            compiler.diagnostics.borrow_mut().log(
                None,
                "Compiler",
                format!(
                    "Unable to return `{}` from a function that returns `{}`",
                    return_type.to_string().bold().yellow(),
                    compiler
                        .bytecode
                        .return_type
                        .clone()
                        .unwrap_or(Primitive::Void)
                        .to_string()
                        .bold()
                        .yellow()
                )
                .bright_red()
                .to_string(),
            );
            panic!()
        }
        return_value.to_bytecode(compiler);
        compiler.bytecode.function.chunk.emit_op(OpCode::Return);
    }
}
