use crate::{common::opcode::OpCode, frontend::compiler::Compiler};

use self::{
    block::Block, call_expr::CallExpr, comparison::Comparison, if_expr::IfExpr,
    variable_assignment::VariableAssignment, while_expr::WhileExpr,
};

use super::{
    identifier::Identifier,
    literal::Literal,
    node::{AsNode, Node},
    BinaryOperation, CompileToBytecode,
};
pub trait AsExpr {
    fn to_expr(self) -> Expression;
}
pub mod block;
pub mod call_expr;
pub mod comparison;
pub mod if_expr;
pub mod variable_assignment;
pub mod while_expr;
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: BinaryOperation,
}
impl AsExpr for BinaryExpr {
    fn to_expr(self) -> Expression {
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
    If(IfExpr),
    While(WhileExpr),
    CallExpr(CallExpr),
    Comparison(Comparison),
}
impl AsNode for Expression {
    fn to_node(self) -> Node {
        Node::Expression(self)
    }
}
impl Expression {
    pub fn as_literal(self) -> Literal {
        let Expression::Literal(literal) = self else {panic!()};
        literal
    }
}
impl Expression {
    pub fn as_block(self) -> Block {
        let Expression::Block(block) = self else {
            panic!()
        };

        block
    }
    pub fn as_binary_expr(self) -> BinaryExpr {
        let Expression::Binary(expr) = self else {
            panic!()
        };
        expr
    }
}
impl CompileToBytecode for Expression {
    fn to_bytecode(&self, compiler: &mut Compiler) {
        match self {
            Expression::CallExpr(call_expr) => call_expr.to_bytecode(compiler),
            Expression::While(while_expr) => while_expr.to_bytecode(compiler),
            Expression::Grouping(inner) => inner.to_bytecode(compiler),
            Expression::Literal(literal) => literal.to_bytecode(compiler),
            Expression::VariableAssignment(var) => var.to_bytecode(compiler),
            Expression::Not(expr) => {
                expr.to_bytecode(compiler);
                compiler.bytecode.function.chunk.emit_op(OpCode::Not);
            }
            Expression::If(if_expr) => if_expr.to_bytecode(compiler),
            Expression::Negate(expr) => {
                expr.to_bytecode(compiler);
                compiler.bytecode.function.chunk.emit_op(OpCode::Negate);
            }
            Expression::Comparison(comparison) => comparison.to_bytecode(compiler),
            Expression::Block(block) => block.to_bytecode(compiler),
            Expression::Identifier(identifier) => identifier.to_bytecode(compiler),
            super::Expression::Binary(binary) => {
                let BinaryExpr { lhs, rhs, op } = binary;
                lhs.to_bytecode(compiler);
                rhs.to_bytecode(compiler);

                let chunk = &mut compiler.bytecode.function.chunk;
                match op {
                    super::BinaryOperation::Add => compiler.bytecode.write_add_op(),
                    super::BinaryOperation::Subtract => compiler.bytecode.write_sub_op(),
                    super::BinaryOperation::Multiply => compiler.bytecode.write_mul_op(),
                    super::BinaryOperation::Divide => compiler.bytecode.write_div_op(),
                }
            }
        }
    }
}
