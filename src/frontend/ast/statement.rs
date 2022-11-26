use crate::common::opcode::OpCode;

use super::{
    expression::{AsExpr, Expression},
    node::AsNode,
    CompileToBytecode,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
}

impl AsNode for Statement {
    fn as_node(self) -> super::node::Node {
        super::node::Node::Statement(self)
    }
}
impl AsExpr for Statement {
    fn as_expr(self) -> Expression {
        match self {
            Statement::Expression(expr) => expr,
            _ => panic!(),
        }
    }
}
impl CompileToBytecode for Statement {
    fn to_bytecode(self, function: &mut crate::common::function::Function) -> () {
        match self {
            Statement::Expression(expr) => {
                expr.to_bytecode(function);
                function.chunk.emit_op(OpCode::Pop)
            },
            Statement::Print(expr) => {
                expr.to_bytecode(function);
                function.chunk.emit_op(OpCode::Print);
            }
        }
    }
}
