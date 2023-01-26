use crate::{
    common::opcode::OpCode,
    frontend::{
        compiler::Compiler,
        scanner::{Token, TokenKind},
    },
};

use self::{
    binary_expr::BinaryExpr, block::Block, call_expr::CallExpr, if_expr::IfExpr,
    variable_assignment::VariableAssignment, while_expr::WhileExpr,
};

use super::{
    identifier::Identifier,
    literal::Literal,
    node::{AsNode, Node},
    CompileToBytecode,
};
pub trait AsExpr {
    fn to_expr(self) -> Expression;
}
pub mod binary_expr;
pub mod block;
pub mod call_expr;
pub mod if_expr;
pub mod variable_assignment;
pub mod while_expr;

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
            Expression::Block(block) => block.to_bytecode(compiler),
            Expression::Identifier(identifier) => identifier.to_bytecode(compiler),
            super::Expression::Binary(binary) => {
                let BinaryExpr { lhs, rhs, op } = binary;
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
    }
}
