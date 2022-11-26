use crate::common::function::Function;

use self::{
    expression::{AsExpr, Expression},
    literal::Literal,
};
pub mod expression;
pub mod literal;
pub mod node;
pub mod statement;
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub trait CompileToBytecode {
    fn to_bytecode(self, function: &mut Function) -> ();
}
