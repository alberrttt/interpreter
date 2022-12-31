pub mod ast;
pub mod compiler;
pub mod file;
pub mod fixedvec;
pub mod parser;
pub mod scanner;
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
