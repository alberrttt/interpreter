use crate::common::opcode::OpCode;

use super::{
    node::{AsNode, Node},
    statement::Statement,
    BinaryOperation, CompileToBytecode,
};
pub trait AsExpr {
    fn as_expr(self) -> Expression;
}
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: BinaryOperation,
}
impl AsExpr for BinaryExpr {
    fn as_expr(self) -> Expression {
        Expression::Binary(self)
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Grouping(Box<Node>),
    Binary(BinaryExpr),
}
impl AsNode for Expression {
    fn as_node(self) -> Node {
        Node::Expression(self)
    }
}

impl Expression {
    pub fn as_binary_expr(self) -> BinaryExpr {
        let Expression::Binary(expr) = self else {
            panic!()
        };
        return expr;
    }
}
impl CompileToBytecode for Expression {
    fn to_bytecode(self, function: &mut crate::common::function::Function) -> () {
        match self {
            Expression::Grouping(inner) => inner.to_bytecode(function),
            super::Expression::Binary(binary) => {
                let BinaryExpr { lhs, rhs, op } = binary;
                lhs.to_bytecode(function);
                rhs.to_bytecode(function);

                let chunk = &mut function.chunk;
                chunk.emit_op(match op {
                    super::BinaryOperation::Add => OpCode::Add,
                    super::BinaryOperation::Subtract => OpCode::Sub,
                    super::BinaryOperation::Multiply => OpCode::Mul,
                    super::BinaryOperation::Divide => OpCode::Div,
                })
            }
        }
    }
}
