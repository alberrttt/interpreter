use std::{fs::File, mem::transmute};

use colored::Colorize;

use super::{
    ast::{
        expression::{AsExpr, BinaryExpr, Expression},
        literal::Literal,
        node::{AsNode, Node},
        statement::Statement,
        BinaryOperation,
    },
    file::FileNode,
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
            TokenKind::Star | TokenKind::Slash => Rule {
                precedence: Precedence::Factor,
                prefix: None,
                infix: Some(Self::binary),
            },
            TokenKind::Dash | TokenKind::Plus => Rule {
                infix: Some(Self::binary),
                prefix: None,
                precedence: Precedence::Term,
            },
            TokenKind::String => Rule {
                precedence: Precedence::None,
                prefix: Some(Self::string),

                infix: None,
            },
            _ => Rule {
                precedence: Precedence::Unimpl,
                infix: None,
                prefix: None,
            },
        }
    }
    pub fn string(&mut self) -> Node {
        Literal::String(self.previous().value.clone()).as_node()
    }
    pub fn binary(&mut self, lhs: Node) -> Node {
        // the problem is somewhere here because
        // lhs could be a binary expression with higher/same precedence
        // ex. 2 / 3 * 7
        // lhs: 2
        let rule = Self::get_rule(self.previous().kind);
        let op = match self.previous().kind {
            TokenKind::Plus => BinaryOperation::Add,
            TokenKind::Dash => BinaryOperation::Subtract,
            TokenKind::Star => BinaryOperation::Multiply,
            TokenKind::Slash => BinaryOperation::Divide,
            _ => panic!(),
        };
        // this sees 3 * 7 as its own expression and not as part of 2 / 3
        let prec: Precedence = unsafe { transmute((rule.precedence as u8) + 1) };
        let rhs = self.precedence(prec);
        // thus
        // lhs: 3 * 7
        // rhs: 2
        // when it should be
        // lhs: 2 / 3
        // rhs: 7
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
        self.advance();
        let previous = self.previous();
        let rule = Self::get_rule(previous.kind);
        #[allow(unused_assignments)]
        let mut expression: Node = Node::None;
        if rule.prefix.is_some() {
            expression = rule.prefix.unwrap()(self);
        } else {
            panic!("expected expression");
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
            if prec >= current_rule.precedence {
                break expression;
            }

            self.advance();
            let previous = self.previous();

            match Self::get_rule(previous.kind).infix {
                None => {}
                Some(infix) => {
                    expression = infix(self, expression);
                }
            }
        }
    }
    pub fn expression(&mut self) -> Node {
        self.precedence(Precedence::None)
    }
    pub fn parse_file(&mut self) -> FileNode {
        let mut file = FileNode::default();
        loop {
            if self.at_end() {
                break;
            }
            file.nodes.push(self.node());
        }
        file
    }

    pub fn node(&mut self) -> Node {
        self.statement()
    }
    pub fn expression_statement(&mut self) -> Node {
        let expr = self.expression();
        self.consume(TokenKind::SemiColon, "Expected ';' after expression");
        Statement::Expression(expr.as_expr()).as_node()
    }
    pub fn statement(&mut self) -> Node {
        match self.current().kind {
            TokenKind::Print => {
                self.advance();
                let node = Statement::Print(self.expression().as_expr()).as_node();
                node
            }
            _ => self.expression_statement(),
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let parser = Parser { tokens, index: 0 };

        parser
    }
    pub fn match_token(&mut self, tk: TokenKind) -> bool {
        if !self.check(tk) {
            return false;
        };
        self.advance();
        return true;
    }
    pub fn check(&mut self, tk: TokenKind) -> bool {
        self.current().kind == tk
    }
    pub fn previous(&mut self) -> &Token {
        &self.tokens[self.index - 1]
    }
    pub fn current(&mut self) -> &Token {
        &self.tokens[self.index]
    }

    pub fn advance(&mut self) -> &Token {
        self.index += 1;
        &self.tokens[self.index - 1]
    }
    pub fn consume(&mut self, kind: TokenKind, err: &str) -> &Token {
        let advance = self.advance();

        if advance.kind.eq(&kind) {
            return advance;
        } else {
            panic!("{}", err)
        }
    }

    pub fn peek(&mut self, distance: usize) -> &Token {
        &self.tokens[self.index - (distance)]
    }
}
