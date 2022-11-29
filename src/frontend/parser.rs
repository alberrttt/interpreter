use std::{mem::transmute, process::id};

use colored::Colorize;

use super::{
    ast::{
        declaration::variable_declaration::VariableDeclaration,
        expression::{AsExpr, BinaryExpr, Expression},
        identifier::Identifier,
        literal::Literal,
        node::{AsNode, Node},
        statement::Statement,
        BinaryOperation,
    },
    file::FileNode,
    scanner::{Token, TokenKind},
    Precedence,
};

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub index: usize,
}
pub struct Rule {
    pub precedence: Precedence,
    pub prefix: Option<fn(&mut Parser, can_assign: bool) -> Node>,
    pub infix: Option<fn(&mut Parser, previous: Node) -> Node>,
}

impl Parser {
    pub fn get_rule(kind: TokenKind) -> Rule {
        match kind {
            TokenKind::Identifier => Rule {
                precedence: Precedence::None,
                prefix: Some(|parser, can_assign| {
                    let name = parser.previous().value.to_string();
                    if can_assign && parser.match_token(TokenKind::Equal) {
                        return Statement::VariableReassignment(
                            Identifier { name },
                            parser.expression().as_expr(),
                        )
                        .as_node();
                    }
                    Identifier { name }.as_node()
                }),
                infix: None,
            },
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
            TokenKind::Equal | TokenKind::SemiColon => Rule {
                precedence: Precedence::None,
                infix: None,
                prefix: None,
            },
            _ => Rule {
                precedence: Precedence::Unimpl,
                infix: None,
                prefix: None,
            },
        }
    }
    pub fn string(&mut self, can_assign: bool) -> Node {
        Literal::String(self.previous().value.clone()).as_node()
    }
    pub fn binary(&mut self, lhs: Node) -> Node {
        let rule = Self::get_rule(self.previous().kind);
        let op = match self.previous().kind {
            TokenKind::Plus => BinaryOperation::Add,
            TokenKind::Dash => BinaryOperation::Subtract,
            TokenKind::Star => BinaryOperation::Multiply,
            TokenKind::Slash => BinaryOperation::Divide,
            _ => panic!(),
        };
        // the precedence is +1 so it'll compile it as the rhs
        let prec: Precedence = unsafe { transmute((rule.precedence as u8) + 1) };
        let rhs = self.precedence(prec);

        Expression::Binary(BinaryExpr {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
        })
        .as_node()
    }
    pub fn number(&mut self, can_assign: bool) -> Node {
        Literal::Number(self.previous().value.parse::<f64>().unwrap()).as_node()
    }
    pub fn at_end(&mut self) -> bool {
        self.index + 1 >= self.tokens.len()
    }
    pub fn precedence(&mut self, prec: Precedence) -> Node {
        self.advance();
        let previous = self.previous();
        let rule = Self::get_rule(previous.kind);
        let can_assign: bool = prec <= Precedence::Assignment;
        #[allow(unused_assignments)]
        let mut expression: Node = Node::None;
        if rule.prefix.is_some() {
            expression = rule.prefix.unwrap()(self, can_assign);
        } else {
            panic!(
                "expected expression run.mng:{}:{}",
                previous.line + 1,
                previous.line_start
            );
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
    pub fn parse_file<'a>(&mut self) -> FileNode<'a> {
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
        Statement::Expression(Box::new(expr)).as_node()
    }
    pub fn token_as_identifier(&mut self) -> Identifier {
        self.advance();
        Identifier {
            name: self.previous().value.to_string(),
        }
    }
    pub fn statement(&mut self) -> Node {
        match self.current().kind {
            TokenKind::Print => {
                self.advance();
                let node = Statement::Print(Box::new(self.expression())).as_node();
                self.consume(TokenKind::SemiColon, "");
                node
            }
            TokenKind::Let => {
                self.advance();
                let identifier = self.token_as_identifier();

                self.consume(TokenKind::Equal, "Expected '=' after variable name");
                let initializer = self.expression().as_expr();
                self.consume(
                    TokenKind::SemiColon,
                    "Expected ';' after variable declaration",
                );

                VariableDeclaration {
                    intializer: initializer,
                    identifier,
                    is_global: true,
                }
                .as_node()
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
