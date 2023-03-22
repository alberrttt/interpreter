pub mod ast;
pub mod bytecode;
pub mod compiler;
pub mod error;
pub mod file;
pub mod fixedvec;
pub mod location;
pub mod parser;
pub mod scanner;
pub use ast::*;
pub mod typesystem;
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Unimpl,
    None,
    Assignment,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Grouping,
}
