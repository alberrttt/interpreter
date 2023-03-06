use self::expression::Expression;

use super::compiler::Compiler;
pub mod declaration;
pub mod expression;
pub mod identifier;
pub mod literal;
pub mod node;
pub mod statement;
pub mod types;
pub trait CompileToBytecode {
    fn to_bytecode(&self, compiler: &mut Compiler);
}
