use crate::{
    common::{opcode::OpCode, value::AsValue},
    frontend::compiler::Compiler,
};

use super::{
    expression::{AsExpr, Expression},
    identifier::Identifier,
    node::{AsNode, Node},
    CompileToBytecode,
};
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Box<Node>),
    VariableReassignment(Identifier, Expression),
    Print(Box<Node>),
}

impl AsNode for Statement {
    fn as_node(self) -> super::node::Node {
        super::node::Node::Statement(self)
    }
}
impl AsExpr for Statement {
    fn as_expr(self) -> Expression {
        match self {
            Statement::Expression(expr) => expr.as_expr(),
            _ => panic!(),
        }
    }
}
impl CompileToBytecode for Statement {
    fn to_bytecode(self, compiler: &mut Compiler) -> () {
        match self {
            Statement::Expression(expr) => {
                expr.to_bytecode(compiler);
                compiler.function.chunk.emit_op(OpCode::Pop)
            }
            Statement::Print(expr) => {
                expr.to_bytecode(compiler);
                compiler.function.chunk.emit_op(OpCode::Print);
            }
            Statement::VariableReassignment(name, initializer) => {
                initializer.to_bytecode(compiler);
                let name = compiler.function.chunk.emit_value(name.name.as_value());
                compiler.function.chunk.emit_op(OpCode::SetGlobal(name))
            }
        }
    }
}

pub trait AsStatement {
    fn as_statement(self) -> Statement;
}
