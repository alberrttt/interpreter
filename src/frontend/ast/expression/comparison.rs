use crate::{common::opcode::OpCode, frontend::ast::CompileToBytecode};

use super::{AsExpr, Expression};

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonKind {
    GreaterEq,
    LessEq,
    Equal,
    NotEqual,
    Greater,
    Less,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    pub kind: ComparisonKind,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}
impl AsExpr for Comparison {
    fn to_expr(self) -> Expression {
        Expression::Comparison(self)
    }
}
impl CompileToBytecode for Comparison {
    fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        self.lhs.to_bytecode(compiler);
        self.rhs.to_bytecode(compiler);
        let mut emit_op = |op: OpCode| compiler.bytecode.function.chunk.emit_op(op);
        match self.kind {
            ComparisonKind::GreaterEq => emit_op(OpCode::GreaterEq),
            ComparisonKind::LessEq => emit_op(OpCode::LessEq),
            ComparisonKind::Greater => emit_op(OpCode::Greater),
            ComparisonKind::Equal => emit_op(OpCode::Equal),
            ComparisonKind::NotEqual => emit_op(OpCode::NotEqual),
            ComparisonKind::Less => emit_op(OpCode::Less),
        }
    }
}
