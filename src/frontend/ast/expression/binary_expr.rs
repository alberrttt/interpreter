use crate::frontend::{ast::node::Node, scanner::TokenKind};

use super::{AsExpr, Expression};

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub op: TokenKind,
}
impl AsExpr for BinaryExpr {
    fn to_expr(self) -> Expression {
        Expression::Binary(self)
    }
}
