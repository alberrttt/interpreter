use crate::common::{opcode::OpCode, value::AsValue};

use self::variable_declaration::VariableDeclaration;

use super::{
    expression::{AsExpr, Expression},
    node::{AsNode, Node},
    CompileToBytecode,
};
pub mod variable_declaration;
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Print(Box<Node>),
    VariableDeclaration(VariableDeclaration),
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
            }
            Statement::Print(expr) => {
                expr.to_bytecode(function);
                function.chunk.emit_op(OpCode::Print);
            }
            Statement::VariableDeclaration(variable_declaration) => {
                variable_declaration.to_bytecode(function)
            }
        }
    }
}

pub trait AsStatement {
    fn as_statement(self) -> Statement;
}
