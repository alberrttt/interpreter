use crate::{
    common::{
        interner::{InternedString, StringInterner},
        value::{AsValue, Value},
    },
    frontend::{
        ast::{node::Node, CompileToBytecode},
        compiler::Compiler,
        scanner::TokenKind,
    },
};

use super::{AsExpr, Expression};

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: TokenKind,
}
impl AsExpr for BinaryExpr {
    fn to_expr(self) -> Expression {
        Expression::Binary(self)
    }
}
impl BinaryExpr {
    pub fn compile_assignment(&self, compiler: &mut Compiler) {
        let lhs = &self.lhs;
        let initializer = &self.rhs;
        initializer.to_bytecode(compiler);
        let Node::Identifier(name) = self.lhs.as_ref() else {
            panic!("{:?}", self)
        };
        let name = &name.value;
        let local = compiler.resolve_local(name);
        if let Some(local) = local {
            if compiler.bytecode.compiling_statement {
                compiler.bytecode.write_set_local_consumes_op(local as u16);
            } else {
                compiler.bytecode.write_set_local_op(local as u16);
            }
            return;
        } else if let Some(arg) = compiler.resolve_up_value(name) {
            compiler.bytecode.write_set_up_value_op(arg as u16);
        }
        let string: Value = (&name.lexeme).to_value();
        let location = compiler.bytecode.function.chunk.emit_value(string);
        compiler.bytecode.write_set_global_op(location);
    }
}
impl CompileToBytecode for BinaryExpr {
    fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        let BinaryExpr { lhs, rhs, op } = self;
        if op.eq(&TokenKind::Equal) {
            self.compile_assignment(compiler);
            return;
        }
        lhs.to_bytecode(compiler);
        rhs.to_bytecode(compiler);

        let chunk = &mut compiler.bytecode.function.chunk;
        match op {
            TokenKind::Plus => compiler.bytecode.write_add_op(),
            TokenKind::Dash => compiler.bytecode.write_sub_op(),
            TokenKind::Star => compiler.bytecode.write_mul_op(),
            TokenKind::Slash => compiler.bytecode.write_div_op(),
            TokenKind::Greater => compiler.bytecode.write_greater_op(),
            TokenKind::GreaterEqual => compiler.bytecode.write_greater_eq_op(),
            TokenKind::Less => compiler.bytecode.write_less_op(),
            TokenKind::LessEqual => compiler.bytecode.write_less_eq_op(),
            TokenKind::EqualEqual => compiler.bytecode.write_equal_op(),
            TokenKind::BangEqual => compiler.bytecode.write_not_equal_op(),
            x => panic!("Invalid binary operator {}", x),
        }
    }
}
