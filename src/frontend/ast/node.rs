use crate::common::{function::Function, opcode::OpCode};

use super::{
    expression::{AsExpr, BinaryExpr, Expression},
    literal::Literal,
    statement::Statement,
    CompileToBytecode,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Expression(Expression),
    Literal(Literal),
    Statement(Statement),
    None,
}

impl Node {
    pub fn as_literal(self) -> Literal {
        let Node::Literal(literal) = self else {
            panic!()
        };

        literal
    }
}
impl AsExpr for Node {
    fn as_expr(self) -> Expression {
        let Node::Expression(expr) = self else {
            panic!("Expected {:?} to be an expression", self)
        };

        expr
    }
}
pub trait AsNode {
    fn as_node(self) -> Node;
}
impl CompileToBytecode for Node {
    // we need it to emit constants
    fn to_bytecode(self, function: &mut Function) -> () {
        match self {
            Node::Expression(expr) => match expr {
                super::Expression::Grouping(inner) => inner.to_bytecode(function),
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
            },
            Node::Statement(statement) => statement.to_bytecode(function),
            Node::Literal(literal) => literal.to_bytecode(function),
            x => x.to_bytecode(function),
        }
    }
}
