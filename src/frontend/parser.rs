use colored::Colorize;

use super::{
    ast::{BinaryExpr, BinaryOperation, Expression, Literal, Node},
    scanner::{Scanner, Token, TokenKind},
    Precedence,
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub index: usize,
}
pub struct Rule {
    pub precedence: Precedence,
    pub prefix: Option<fn(&mut Parser) -> Node>,
    pub infix: Option<fn(&mut Parser, previous: Node) -> Node>,
}

impl Parser {
    pub fn get_rule(kind: TokenKind) -> Rule {
        match kind {
            TokenKind::Number => Rule {
                precedence: Precedence::None,
                infix: None,
                prefix: Some(Self::number),
            },
            TokenKind::Dash | TokenKind::Plus => Rule {
                infix: Some(Self::binary),
                prefix: None,
                precedence: Precedence::Term,
            },
            _ => Rule {
                precedence: Precedence::Unimpl,
                infix: None,
                prefix: None,
            },
        }
    }
    pub fn binary(&mut self, lhs: Node) -> Node {
        let op = match self.previous().kind {
            TokenKind::Plus => BinaryOperation::Add,
            TokenKind::Dash => BinaryOperation::Subtract,
            TokenKind::Star => BinaryOperation::Multiply,
            TokenKind::Slash => BinaryOperation::Divide,
            _ => panic!(),
        };
        let rhs = self.expression();

        Expression::Binary(BinaryExpr {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
        })
        .as_node()
    }
    pub fn number(&mut self) -> Node {
        Literal::Number(self.previous().value.parse::<f64>().unwrap()).as_node()
    }
    pub fn at_end(&mut self) -> bool {
        self.index + 1 >= self.tokens.len()
    }
    pub fn precedence(&mut self, prec: Precedence) -> Node {
        self.next();
        let previous = self.previous();
        let rule = Self::get_rule(previous.kind);
        #[allow(unused_assignments)]
        let mut expression: Node = Node::None;
        if rule.prefix.is_some() {
            expression = rule.prefix.unwrap()(self);
        } else {
            panic!("Expect expression");
        }

        loop {
            if self.at_end() {
                break expression;
            }
            let current = self.current();
            let current_rule = Self::get_rule(current.kind);
            if current_rule.precedence == Precedence::Unimpl && cfg!(debug_assertions) {
                println!(
                    "{} {}",
                    format!("Unimplemented rule:").bold().on_red().yellow(),
                    current.kind
                );
            }
            if prec > current_rule.precedence {
                break expression;
            }

            self.next();
            let previous = self.previous();

            match Self::get_rule(previous.kind).infix {
                None => {}
                Some(infix) => {
                    expression = infix(self, expression);
                }
            }
            return expression;
        }
    }
    pub fn expression(&mut self) -> Node {
        self.precedence(Precedence::None)
    }
    pub fn parse(&mut self) -> Node {
        self.expression()
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let parser = Parser { tokens, index: 0 };

        parser
    }
    pub fn previous(&mut self) -> &Token {
        &self.tokens[self.index - 1]
    }
    pub fn current(&mut self) -> &Token {
        &self.tokens[self.index]
    }

    pub fn next(&mut self) -> &Token {
        self.index += 1;
        &self.tokens[self.index - 1]
    }
    pub fn expect(&mut self, kind: TokenKind) -> &Token {
        let next = self.next();

        if next.kind.eq(&kind) {
            return next;
        } else {
            panic!()
        }
    }

    pub fn peek(&mut self) -> &Token {
        &self.tokens[self.index + 1]
    }
}
