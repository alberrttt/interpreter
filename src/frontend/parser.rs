/// the parser will make an ast
use std::{mem::transmute, ops::Range, rc::Rc};

use colored::Colorize;

use crate::cli_context::Context;

use super::{
    ast::{
        declaration::{
            function::FunctionDeclaration, variable_declaration::VariableDeclaration, AsDeclaration,
        },
        expression::{
            block::Block,
            comparison::{Comparison, ComparisonKind},
            if_expr::IfExpr,
            variable_assignment::VariableAssignment,
            while_expr::WhileExpr,
            AsExpr, BinaryExpr, Expression,
        },
        identifier::Identifier,
        literal::Literal,
        node::{AsNode, Node},
        statement::{return_stmt::ReturnStmt, Statement},
        BinaryOperation,
    },
    compiler::{Compiler, FunctionType},
    file::FileNode,
    scanner::{Position, Scanner, Token, TokenKind},
    Precedence,
};

#[derive(Debug)]
pub struct CompilerRef<'a>(pub *const Compiler<'a>);
impl<'a> CompilerRef<'a> {
    pub fn as_ref(&self) -> &Compiler<'a> {
        unsafe { &*self.0 }
    }
}
#[derive(Debug)]
pub struct Parser<'a> {
    pub context: Option<&'a mut Context<'a>>,
    pub had_error: bool,
    pub scanner: Box<Scanner>,
    pub panic_mode: bool,
    pub scope_depth: usize,
    pub compiler: CompilerRef<'a>,
    pub previous: Rc<Token>,
    pub current: Token,
    pub tokens: Vec<Token>,
    pub index: usize,
}
pub struct Rule<'a> {
    pub precedence: Precedence,
    pub prefix: Option<fn(&mut Parser<'a>, can_assign: bool) -> Node>,
    pub infix: Option<fn(&mut Parser<'a>, previous: Node) -> Node>,
}

impl<'a> Parser<'a> {
    pub fn get_rule(kind: TokenKind) -> Rule<'a> {
        match kind {
            TokenKind::LeftParen => Rule {
                precedence: Precedence::Grouping,
                prefix: Some(|parser: &mut Parser, can_assign: bool| {
                    Expression::Grouping(Box::new(parser.expression().unwrap().as_expr())).as_node()
                }),
                infix: None,
            },
            TokenKind::While => Rule {
                precedence: Precedence::None,
                prefix: Some(Self::while_expr),
                infix: None,
            },
            TokenKind::Greater
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual => Rule {
                precedence: Precedence::Comparison,
                prefix: None,
                infix: Some(|parser: &mut Parser, lhs: Node| {
                    let comparison_token = parser.previous().kind;
                    Comparison {
                        lhs: Box::new(lhs.as_expr()),
                        rhs: Box::new(parser.expression().unwrap().as_expr()),
                        kind: comparison_token.try_into().unwrap(),
                    }
                    .as_expr()
                    .as_node()
                }),
            },
            TokenKind::If => Rule {
                precedence: Precedence::None,
                prefix: Some(Self::if_expr),
                infix: None,
            },
            TokenKind::LeftBrace => Rule {
                precedence: Precedence::None,
                prefix: Some(Self::block),
                infix: None,
            },
            TokenKind::True => Rule {
                precedence: Precedence::None,
                prefix: Some(|_, _| Literal::Bool(true).as_node()),
                infix: None,
            },
            TokenKind::False => Rule {
                precedence: Precedence::None,
                prefix: Some(|_, _| Literal::Bool(false).as_node()),
                infix: None,
            },
            TokenKind::Identifier => Rule {
                precedence: Precedence::None,
                prefix: Some(|parser, can_assign| {
                    let token = parser.previous().clone();
                    let _global = if parser.scope_depth > 0 { false } else { true };
                    if can_assign && parser.match_token(TokenKind::Equal) {
                        return Expression::VariableAssignment(VariableAssignment {
                            name: Identifier { value: token },
                            initializer: Box::new(parser.expression().unwrap().as_expr()),
                        })
                        .as_node();
                    }
                    Identifier { value: token }.as_node()
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
            TokenKind::Plus => Rule {
                infix: Some(Self::binary),
                prefix: None,
                precedence: Precedence::Term,
            },
            TokenKind::Dash => Rule {
                infix: Some(Self::binary),
                prefix: Some(|parser, _can_assign| {
                    Expression::Negate(Box::new(
                        parser.precedence(Precedence::Unary).unwrap().as_expr(),
                    ))
                    .as_node()
                }),
                precedence: Precedence::Term,
            },
            TokenKind::Bang => Rule {
                precedence: Precedence::Unary,
                prefix: Some(|parser, can_assign| {
                    Expression::Not(Box::new(
                        parser.precedence(Precedence::Unary).unwrap().as_expr(),
                    ))
                    .as_node()
                }),
                infix: None,
            },
            TokenKind::String => Rule {
                precedence: Precedence::None,
                prefix: Some(Self::string),

                infix: None,
            },
            TokenKind::Equal | TokenKind::SemiColon | TokenKind::Comma => Rule {
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

    pub fn at_end(&mut self) -> bool {
        self.scanner.at_end()
    }
    pub fn precedence(&mut self, prec: Precedence) -> Result<Node, String> {
        self.advance();
        let _path = self.context.as_ref().unwrap().file_path.to_str().unwrap();
        let previous = self.previous();
        let rule = Self::get_rule(previous.kind);
        let can_assign: bool = prec <= Precedence::Assignment;
        #[allow(unused_assignments)]
        let mut expression: Node = Node::None;
        if rule.prefix.is_some() {
            expression = rule.prefix.unwrap()(self, can_assign);
        } else {
            return Err(format!(
                "no expr {}:{}:{}",
                self.context
                    .as_ref()
                    .unwrap()
                    .file_path
                    .as_os_str()
                    .to_str()
                    .unwrap(),
                previous.position.line + 1,
                previous.position.start_in_line + 1,
            ));
        }

        loop {
            if self.at_end() {
                break Ok(expression);
            }
            let current = self.current();
            let current_rule = Self::get_rule(current.kind);
            // if current_rule.precedence == Precedence::Unimpl && cfg!(debug_assertions) {
            //     println!(
            //         "{} {}",
            //         format!("Unimplemented rule:").bold().on_red().yellow(),
            //         current.kind
            //     );
            // }
            if prec >= current_rule.precedence {
                break Ok(expression);
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
    pub fn expression(&mut self) -> Result<Node, String> {
        self.precedence(Precedence::None)
    }
    pub fn parse_file(&mut self) -> FileNode<'a> {
        let mut file = FileNode::default();
        self.advance();
        loop {
            if self.at_end() {
                break;
            }
            file.nodes.push(self.node());
        }
        file
    }

    pub fn node(&mut self) -> Node {
        let node = self.statement();
        if self.panic_mode {
            self.synchronize();
        }
        node
    }
    pub fn expression_statement(&mut self) -> Node {
        let expr = self.expression().unwrap().as_expr();
        self.consume(TokenKind::SemiColon, "Expected ';' after expression");
        Statement::Expression(expr).as_node()
    }
    pub fn token_as_identifier(&mut self) -> Identifier {
        self.advance();
        Identifier {
            value: self.previous().clone(),
        }
    }
    pub fn statement(&mut self) -> Node {
        match self.current().kind {
            TokenKind::Print => {
                self.advance();
                let node = Statement::Print(Box::new(self.expression().unwrap())).as_node();
                self.consume(TokenKind::SemiColon, "Expected ';' ");
                node
            }
            TokenKind::AssertEq => {
                self.advance();
                let lhs = self.expression().unwrap().as_expr();
                self.consume(TokenKind::Comma, "Expected ','' to seperate lhs and rhs");
                let rhs = self.expression().unwrap().as_expr();
                self.consume(TokenKind::SemiColon, "Expected ';'");

                let node = Statement::AssertEq(lhs, rhs);
                node.as_node()
            }
            TokenKind::AssertNe => {
                self.advance();
                let lhs = self.expression().unwrap().as_expr();
                self.consume(TokenKind::Comma, "Expected ','' to seperate lhs and rhs");
                let rhs = self.expression().unwrap().as_expr();
                self.consume(TokenKind::SemiColon, "Expected ';'");

                let node = Statement::AssertNe(lhs, rhs);
                node.as_node()
            }
            TokenKind::Let => {
                self.advance();
                let identifier = self.token_as_identifier();
                self.consume(TokenKind::Equal, "Expected '=' after variable name");
                let initializer = self.expression().unwrap().as_expr();
                self.consume(
                    TokenKind::SemiColon,
                    "Expected ';' after variable declaration",
                );

                VariableDeclaration {
                    intializer: initializer,
                    identifier,
                }
                .as_node()
            }
            TokenKind::Func => {
                self.advance();
                let identifier = self.token_as_identifier();
                self.consume(TokenKind::LeftParen, "err");
                self.consume(TokenKind::RightParen, "err");
                self.consume(TokenKind::LeftBrace, "Expected '{'");
                FunctionDeclaration {
                    name: identifier,
                    block: self.block(false).as_expr().as_block(),
                }
                .as_declaration()
                .as_node()
            }
            TokenKind::Return => {
                self.advance();
                if self.scope_depth == 0
                    && self
                        .compiler
                        .as_ref()
                        .function_type
                        .eq(&FunctionType::Script)
                {
                    self.error("Cannot return from the top level of a script")
                }
                if let Ok(expr) = self.expression() {
                    let expr = expr.as_expr();
                    self.consume(TokenKind::SemiColon, "Expected ';' after expression");
                    return Statement::Return(ReturnStmt { expr: Some(expr) }).as_node();
                } else {
                    self.consume(TokenKind::SemiColon, "Expected ';' after return statement");
                    return Statement::Return(ReturnStmt { expr: None }).as_node();
                }
            }
            _ => self.expression_statement(),
        }
    }
}
impl Parser<'_> {
    pub fn while_expr(&mut self, _can_assign: bool) -> Node {
        let condition = self.expression().unwrap().as_expr();
        let block = self.expression().unwrap().as_expr().as_block();

        WhileExpr {
            predicate: Box::new(condition),
            block,
        }
        .as_expr()
        .as_node()
    }
    pub fn if_expr(&mut self, _can_assign: bool) -> Node {
        let condition = self.expression().unwrap().as_expr();
        let then = self.expression().unwrap().as_expr().as_block();
        #[allow(unused_mut)]
        let mut else_block = None;
        if self.match_token(TokenKind::Else) {
            let block = self.expression().unwrap().as_expr().as_block();
            else_block = Some(block)
        }
        IfExpr {
            predicate: Box::new(condition),
            then,
            else_block,
        }
        .as_expr()
        .as_node()
    }
    pub fn block(&mut self, _can_assign: bool) -> Node {
        self.begin_scope();
        let mut block = Block {
            declarations: Vec::new(),
        };
        loop {
            if !self.check(TokenKind::RightBrace) && !self.check(TokenKind::EOF) {
                block.declarations.push(self.node())
            } else {
                break;
            }
        }
        self.consume(TokenKind::RightBrace, "Expected '}' after block to close");
        self.end_scope();
        return block.as_node();
    }
    pub fn string(&mut self, _can_assign: bool) -> Node {
        Literal::String(self.previous().lexeme.clone()).as_node()
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
        let rhs = self.precedence(prec).unwrap();

        Expression::Binary(BinaryExpr {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
        })
        .as_node()
    }
    pub fn number(&mut self, _can_assign: bool) -> Node {
        Literal::Number(self.previous().lexeme.parse::<f64>().unwrap()).as_node()
    }
}
const EOF: &Token = &Token {
    kind: TokenKind::EOF,
    lexeme: String::new(),
    line: 0,
    length: 0,
    position: Position {
        line: 0,
        start_in_source: 0,
        start_in_line: 0,
    },
};
impl<'a> Parser<'a> {
    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }
    pub fn end_scope(&mut self) {
        self.scope_depth -= 1;
    }
    pub fn error(&mut self, msg: &str) {
        self.had_error = true;

        let previous = self.previous().to_owned();
        self.error_at(&previous, msg);
    }
    pub fn error_at_current(&mut self, msg: &str) {
        self.had_error = true;
        let current = self.current().to_owned();
        self.error_at(&current, msg)
    }
    pub fn synchronize(&mut self) {
        self.panic_mode = false;
        loop {
            if self.current().kind.eq(&TokenKind::EOF) {
                return;
            }
            if self.previous().kind.eq(&TokenKind::SemiColon) {
                return;
            };
            println!("{}", self.current().kind);
            match self.current().kind {
                TokenKind::Return | TokenKind::Print | TokenKind::Func | TokenKind::Let => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }
    pub fn error_at(&mut self, token: &Token, msg: &str) {
        self.panic_mode = true;
        let diagnostics = &mut self.context.as_mut().unwrap().diagnostics;

        match token.kind {
            TokenKind::EOF => {
                diagnostics.log(
                    Some(&token.position),
                    "Compiler",
                    format!("Error at EOF: ",),
                );
            }

            _ => {
                let range: Range<usize> = (token.position.start_in_source as usize)
                    ..(token.position.start_in_source as usize + token.length as usize);
                diagnostics.log(
                    Some(&token.position),
                    "Compiler",
                    format!("Error at `{}`: ", self.scanner.source[range].to_string()),
                );
            }
        }
        println!("{}", msg.red());
    }
    pub fn new(
        scanner: Box<Scanner>,
        context: Option<&'a mut Context<'a>>,
        compiler: *const Compiler<'a>,
    ) -> Parser<'a> {
        Parser {
            compiler: CompilerRef(compiler),
            context,
            scanner: scanner,
            had_error: false,
            panic_mode: false,
            scope_depth: 0,
            current: EOF.to_owned(),
            previous: Rc::new(EOF.to_owned()),
            tokens: Vec::new(),
            index: 0,
        }
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
    pub fn previous(&self) -> &Token {
        &self.previous
    }
    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn consume(&mut self, kind: TokenKind, err: &str) {
        let current = self.current().kind;
        if current.ne(&kind) {
            self.error_at_current(err);
        }

        self.advance();
    }

    pub fn peek(&mut self, distance: usize) -> &Token {
        &self.scanner.tokens[self.scanner.tokens.len() - (1 + distance)]
    }
}

impl<'a> Parser<'a> {
    pub fn advance(&mut self) -> &Token {
        let current = self.scanner.next();
        self.tokens.insert(self.index, self.current.to_owned());
        self.previous = Rc::new(self.tokens[self.index].to_owned());
        self.current = current;
        self.index += 1;
        &self.current
    }
}
