use crate::{
    common::opcode::OpCode,
    frontend::{
        compiler::Compiler,
        scanner::{Token, TokenKind},
    },
};

use self::{
    binary_expr::BinaryExpr, block::Block, call_expr::Call, if_expr::If, while_expr::While,
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
pub mod while_expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Grouping(Box<Expression>),
    Binary(BinaryExpr),
    Literal(Literal),
    Not(Box<Expression>),
    Negate(Box<Expression>),
    Block(Block),
    Identifier(Identifier),
    If(If),
    While(While),
    CallExpr(Call),
    None
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
            Expression::None => {
                // fix later
                compiler.bytecode.write_void_op()
            }
            Expression::CallExpr(call_expr) => call_expr.to_bytecode(compiler),
            Expression::While(while_expr) => while_expr.to_bytecode(compiler),
            Expression::Grouping(inner) => inner.to_bytecode(compiler),
            Expression::Literal(literal) => literal.to_bytecode(compiler),
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
            super::Expression::Binary(binary) => binary.to_bytecode(compiler),
        }
    }
}
