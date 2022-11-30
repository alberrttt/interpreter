

use self::expression::Expression;

use super::compiler::{Compiler};
pub mod declaration;
pub mod expression;
pub mod identifier;
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
    fn to_bytecode(self, compiler: &mut Compiler) -> ();
}
