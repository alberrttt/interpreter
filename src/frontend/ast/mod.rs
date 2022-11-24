use std::ops::Add;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
}
impl Literal {
    pub fn as_node(self) -> Node {
        Node::Literal(self)
    }
}
#[derive(Debug, PartialEq)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: BinaryOperation,
}
#[derive(Debug, PartialEq)]
pub enum Expression {
    Grouping(Box<Node>),
    Binary(BinaryExpr),
}

impl Expression {
    pub fn as_node(self) -> Node {
        Node::Expression(self)
    }
    pub fn as_binary_expr(self) -> BinaryExpr {
        let Expression::Binary(expr) = self else {
            panic!()
        };
        return expr;
    }
}
#[derive(Debug, PartialEq)]
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
