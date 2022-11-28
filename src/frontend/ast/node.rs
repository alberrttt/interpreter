use crate::{
    common::{function::Function, opcode::OpCode},
    frontend::compiler::Compiler,
};

use super::{
    declaration::{self, Declaration},
    expression::{AsExpr, BinaryExpr, Expression},
    identifier::{self, Identifier},
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
            Node::Identifier(_) => todo!(),
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
            function,
            scanner,
            parser,
            context,
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
