use colored::Colorize;

use super::{
    ast::{BinaryOperation, Expression, Literal, Node},
    scanner::{Scanner, Token, TokenKind},
    Precedence,
};

pub struct Parser {
    pub token: *mut Token,
    pub token_start: *mut Token,
    pub len: usize,
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

        Expression::Binary {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
        }
        .as_node()
    }
    pub fn number(&mut self) -> Node {
        Literal::Number(self.previous().value.parse::<f64>().unwrap()).as_node()
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
    pub fn new(scanner: &mut Scanner) -> Parser {
        let slice = scanner.tokens.as_mut_slice();
        let parser = Parser {
            token: slice.as_mut_ptr(),
            token_start: slice.as_mut_ptr(),
            len: slice.len(),
        };

        parser
    }
    pub fn previous(&mut self) -> &Token {
        self.check();

        unsafe { &*self.token.offset(-1) }
    }
    pub fn current(&mut self) -> &Token {
        self.check();
        let token = unsafe { &*self.token };
        token
    }
    pub fn next(&mut self) -> &Token {
        let token = unsafe { &*self.token };
        self.check();

        unsafe { self.token = self.token.offset(1) };

        token
    }
    pub fn expect(&mut self, kind: TokenKind) -> &Token {
        self.check();
        let next = self.next();

        if next.kind.eq(&kind) {
            return next;
        } else {
            panic!()
        }
    }
    #[inline]
    fn check(&mut self) {
        unsafe {
            if self.token.is_null() || self.token_start.is_null() {
                panic!()
            }
            let offset = self.token_start.offset_from(self.token);
            if offset > self.len as isize {
                panic!()
            }
        }
    }
    pub fn peek(&mut self) -> &Token {
        self.check();
        unsafe { &*self.token.offset(1) }
    }
}
