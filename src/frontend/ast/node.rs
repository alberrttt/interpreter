use crate::frontend::{compiler::Compiler, scanner::Position};

use super::{
    declaration::Declaration,
    expression::{AsExpr, Expression},
    identifier::Identifier,
    literal::Literal,
    statement::Statement,
    CompileToBytecode,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Expression(Expression),
    Literal(Literal),
    Statement(Statement),
    Declaration(Declaration),
    Identifier(Identifier),
    None,
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
    fn as_expr(self) -> Expression {
        match self {
            Node::Expression(expr) => expr,
            Node::Literal(literal) => Expression::Literal(literal),
            Node::Declaration(_) => panic!(),
            Node::Statement(_) => panic!(),
            Node::Identifier(identifier) => Expression::Identifier(identifier),
            Node::None => panic!(),
        }
    }
}
pub trait AsNode {
    fn as_node(self) -> Node;
}
impl CompileToBytecode for Node {
    // we need it to emit constants
    fn to_bytecode(self, compiler: &mut Compiler) -> () {
        let Compiler {
            function: _,
            scanner: _,
            parser: _,
            context: _,
            scope_depth: _,
            enclosing: _,
            locals: _,
        } = compiler;
        match self {
            Node::Expression(expr) => expr.to_bytecode(compiler),
            Node::Statement(statement) => statement.to_bytecode(compiler),
            Node::Identifier(identifier) => identifier.to_bytecode(compiler),
            Node::Literal(literal) => literal.to_bytecode(compiler),
            Node::Declaration(declaration) => declaration.to_bytecode(compiler),
            _ => unimplemented!(),
        }
    }
}

pub trait AstPosition {
    fn position(&self) -> &Position;
}
