use crate::{common::opcode::OpCode, frontend::compiler::Compiler};

use self::{block::Block, variable_assignment::VariableAssignment};

use super::{
    identifier::Identifier,
    literal::Literal,
    node::{AsNode, Node},
    BinaryOperation, CompileToBytecode,
};
pub trait AsExpr {
    fn as_expr(self) -> Expression;
}
pub mod block;
pub mod variable_assignment;
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
    Grouping(Box<Expression>),
    Binary(BinaryExpr),
    Literal(Literal),
    VariableAssignment(VariableAssignment),
    Not(Box<Expression>),
    Negate(Box<Expression>),
    Block(Block),
    Identifier(Identifier),
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
    fn to_bytecode(self, compiler: &mut Compiler) -> () {
        match self {
            Expression::Grouping(inner) => inner.to_bytecode(compiler),
            Expression::Literal(literal) => literal.to_bytecode(compiler),
            Expression::VariableAssignment(var) => var.to_bytecode(compiler),
            Expression::Not(expr) => {
                expr.to_bytecode(compiler);
                compiler.function.chunk.emit_op(OpCode::Not);
            }
            Expression::Negate(expr) => {
                expr.to_bytecode(compiler);
                compiler.function.chunk.emit_op(OpCode::Negate);
            }
            Expression::Block(block) => block.to_bytecode(compiler),
            Expression::Identifier(identifier) => identifier.to_bytecode(compiler),
            super::Expression::Binary(binary) => {
                let BinaryExpr { lhs, rhs, op } = binary;
                lhs.to_bytecode(compiler);
                rhs.to_bytecode(compiler);

                let chunk = &mut compiler.function.chunk;
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
