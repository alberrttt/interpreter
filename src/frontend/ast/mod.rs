use std::ops::Add;

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
}
impl Literal {
    pub fn as_node(self) -> Node {
        Node::Literal(self)
    }
}
#[derive(Debug)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(Debug)]
pub enum Expression {
    Grouping(Box<Node>),
    Binary {
        lhs: Box<Node>,
        rhs: Box<Node>,
        op: BinaryOperation,
    },
}
impl Expression {
    pub fn as_node(self) -> Node {
        Node::Expression(self)
    }
}
#[derive(Debug)]
pub enum Node {
    Expression(Expression),
    Literal(Literal),
    None,
}

impl Node {
    pub fn as_literal(self) -> Literal {
        let Node::Literal(literal) = self else {
            panic!()
        };

        literal
    }
    pub fn as_expr(self) -> Expression {
        let Node::Expression(expr) = self else {
            panic!("Expected {:?} to be an expression", self)
        };

        expr
    }
}
