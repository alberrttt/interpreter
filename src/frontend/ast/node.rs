use std::fmt::Debug;

use crate::frontend::{compiler::Compiler, scanner::Position};

use super::{
    declaration::Declaration,
    expression::{AsExpr, Expression},
    identifier::Identifier,
    literal::Literal,
    statement::Statement,
    CompileToBytecode,
};

#[derive(Clone)]
pub enum Node {
    Expression(Expression),
    Literal(Literal),
    Statement(Statement),
    Declaration(Declaration),
    Identifier(Identifier),
    None,
    /// use this one if you don't want the node to be emitted
    Empty,
    Emit(fn(compiler: &mut Compiler) -> ()),
}
impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(arg0) => f.debug_tuple("Expression").field(arg0).finish(),
            Self::Literal(arg0) => f.debug_tuple("Literal").field(arg0).finish(),
            Self::Statement(arg0) => f.debug_tuple("Statement").field(arg0).finish(),
            Self::Declaration(arg0) => f.debug_tuple("Declaration").field(arg0).finish(),
            Self::Identifier(arg0) => f.debug_tuple("Identifier").field(arg0).finish(),
            Self::None => write!(f, "None"),
            Self::Empty => write!(f, "Empty"),
            Self::Emit(_arg0) => f.debug_tuple("Emit").finish(),
        }
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Expression(l0), Self::Expression(r0)) => l0 == r0,
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            (Self::Statement(l0), Self::Statement(r0)) => l0 == r0,
            (Self::Declaration(l0), Self::Declaration(r0)) => l0 == r0,
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            // (Self::Emit(l0), Self::Emit(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
impl Node {
    pub fn as_identifier(self) -> Identifier {
        let Node::Identifier(identifier) = self else {
            panic!("{:?}", self)
        };

        identifier
    }
    pub fn as_literal(self) -> Literal {
        let Node::Literal(literal) = self else {
            panic!()
        };

        literal
    }
}
impl AsExpr for Node {
    fn to_expr(self) -> Expression {
        match self {
            Node::Expression(expr) => expr,
            Node::Literal(literal) => Expression::Literal(literal),
            Node::Declaration(_) => unimplemented!(),
            Node::Statement(_) => unimplemented!(),
            Node::Identifier(identifier) => Expression::Identifier(identifier),
            Node::None => panic!(),
            Node::Empty => panic!(),
            Node::Emit(_) => panic!(),
        }
    }
}
pub trait AsNode {
    fn to_node(self) -> Node;
}
impl CompileToBytecode for Node {
    // we need it to emit constants
    fn to_bytecode(&self, compiler: &mut Compiler) {
        let _function = &mut compiler.function;
        match self {
            Node::Expression(expr) => expr.to_bytecode(compiler),
            Node::Statement(statement) => statement.to_bytecode(compiler),
            Node::Identifier(identifier) => identifier.to_bytecode(compiler),
            Node::Literal(literal) => literal.to_bytecode(compiler),
            Node::Declaration(declaration) => declaration.to_bytecode(compiler),
            Node::Emit(emit) => emit(compiler),
            _ => unimplemented!(),
        }
    }
}

pub trait AstPosition {
    fn position(&self) -> &Position;
}
