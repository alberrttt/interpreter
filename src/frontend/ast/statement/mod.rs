use crate::{common::opcode::OpCode, frontend::compiler::Compiler};
pub mod return_stmt;
use self::return_stmt::ReturnStmt;

use super::{
    expression::{AsExpr, Expression},
    node::{AsNode, Node},
    CompileToBytecode,
};
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Print(Box<Node>),
    AssertEq(Expression, Expression),
    AssertNe(Expression, Expression),
    Return(ReturnStmt),
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
    fn to_bytecode(self, compiler: &mut Compiler) -> () {
        match self {
            Statement::Return(return_stmt) => return_stmt.to_bytecode(compiler),
            Statement::Expression(expr) => {
                expr.to_bytecode(compiler);
                compiler.function.chunk.emit_op(OpCode::Pop)
            }
            Statement::Print(expr) => {
                expr.to_bytecode(compiler);
                compiler.function.chunk.emit_op(OpCode::Print);
            }
            Statement::AssertEq(lhs, rhs) => {
                lhs.to_bytecode(compiler);
                rhs.to_bytecode(compiler);

                compiler.function.chunk.emit_op(OpCode::AssertEq)
            }
            Statement::AssertNe(lhs, rhs) => {
                lhs.to_bytecode(compiler);
                rhs.to_bytecode(compiler);

                compiler.function.chunk.emit_op(OpCode::AssertNe)
            }
        }
    }
}

pub trait AsStatement {
    fn as_statement(self) -> Statement;
}
