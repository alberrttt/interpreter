/// the parser will make an ast
use std::{alloc::Layout, cell::RefCell, mem::transmute, ops::Range, rc::Rc};

use clap::parser;
use colored::Colorize;

use crate::backend::vm::natives::MACROS::*;
use crate::frontend::{
    ast::{
        declaration::{
            function::FunctionDeclaration, variable_declaration::VariableDeclaration, AsDeclaration,
        },
        expression::{
            binary_expr::BinaryExpr, block::Block, call_expr::Call, if_expr::If, while_expr::While,
            AsExpr, Expression,
        },
        identifier::Identifier,
        literal::Literal,
        node::{AsNode, EmitFn, Node},
        statement::{return_stmt::ReturnStmt, Statement},
    },
    compiler::{Compiler, FunctionType},
    file::FileNode,
    scanner::{Position, Scanner, Token, TokenKind},
    Precedence,
};
use crate::{cli_helper::Diagnostics, common::opcode::OpCode, frontend::ast::CompileToBytecode};

use super::ast::literal::Literals;

#[derive(Debug)]
pub struct CompilerRef<'a>(pub *const Compiler<'a>);
impl<'a> CompilerRef<'a> {
    pub fn into(&self) -> &Compiler<'a> {
        #[allow(unsafe_code)]
        unsafe {
            &*self.0
        }
    }
}

#[derive(Debug, Default)]
pub struct Parser<'a> {
    pub diagnostics: Rc<RefCell<Diagnostics<'a>>>,
    pub scanner: Scanner,
    pub had_error: bool,
    pub panic_mode: bool,
    pub scope_depth: usize,
    pub function_type: FunctionType,
    pub token_state: TokenState,
}

#[derive(Debug, Default)]
pub struct TokenState {
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
            TokenKind::Hash => Rule {
                precedence: Precedence::None,
                prefix: Some(|parser, _can_assign| {
                    parser.advance();
                    let ident = parser.previous();
                    if ident.lexeme.eq("void") {
                        return EmitFn(Box::new(|compiler| {
                            compiler.bytecode.write_void_op();
                        }))
                        .into();
                    }
                    panic!()
                }),
                infix: None,
            },
            TokenKind::LeftParen => Rule {
                precedence: Precedence::Grouping,
                prefix: Some(|parser: &mut Parser, _can_assign: bool| {
                    let expr =
                        Expression::Grouping(Box::new(parser.expression().unwrap().to_expr()))
                            .to_node();
                    parser.consume(TokenKind::RightParen, "expected right parenthesis to close");
                    expr
                }),
                infix: Some(Self::call_expr),
            },
            TokenKind::While => Rule {
                precedence: Precedence::None,
                prefix: Some(Self::while_expr),
                infix: None,
            },
            TokenKind::Equal => Rule {
                precedence: Precedence::Assignment,
                prefix: None,
                infix: Some(Self::binary),
            },
            TokenKind::Greater
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual
            | TokenKind::EqualEqual
            | TokenKind::BangEqual => Rule {
                precedence: Precedence::Comparison,
                prefix: None,
                infix: Some(Self::binary),
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
                prefix: Some(|p, _| Literal(Literals::Bool(true), p.previous().clone()).as_node()),
                infix: None,
            },
            TokenKind::False => Rule {
                precedence: Precedence::None,
                prefix: Some(|p, _| Literal(Literals::Bool(false), p.previous().clone()).as_node()),
                infix: None,
            },
            TokenKind::Identifier => Rule {
                precedence: Precedence::None,
                prefix: Some(Self::identifier),
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
                        parser.precedence(Precedence::Unary).unwrap().to_expr(),
                    ))
                    .to_node()
                }),
                precedence: Precedence::Term,
            },
            TokenKind::Bang => Rule {
                precedence: Precedence::Unary,
                prefix: Some(|parser, _can_assign| {
                    Expression::Not(Box::new(
                        parser.precedence(Precedence::Unary).unwrap().to_expr(),
                    ))
                    .to_node()
                }),
                infix: None,
            },
            TokenKind::String => Rule {
                precedence: Precedence::None,
                prefix: Some(Self::string),

                infix: None,
            },
            TokenKind::SemiColon | TokenKind::Comma => Rule {
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
        self.current().kind.eq(&TokenKind::EOF)
    }
    pub fn precedence(&mut self, prec: Precedence) -> Result<Node, String> {
        self.advance();
        let _path = {
            let binding = self.diagnostics.borrow();
            binding.file_path.to_str().unwrap()
        };
        let previous = self.previous();
        let rule = Self::get_rule(previous.kind);
        let can_assign: bool = prec <= Precedence::Assignment;
        #[allow(unused_assignments)]
        let mut expression: Node = Node::None;
        if rule.prefix.is_some() {
            expression = rule.prefix.unwrap()(self, can_assign);
        } else {
            return Err(format!(
                "{} no expr {}:{}:{}",
                previous.kind,
                _path,
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
            let node = self.node();

            if node.eq(&Node::Empty) {
            } else {
                file.nodes.push(node);
            }
        }
        file
    }

    pub fn node(&mut self) -> Node {
        let node = match self.current().kind {
            TokenKind::Hash => {
                self.advance();
                if self.match_token(TokenKind::Identifier) {
                    match self.previous().lexeme.as_str() {
                        "void" => {
                            return EmitFn(Box::new(|compiler| {
                                compiler.bytecode.function.chunk.emit_op(OpCode::Void);
                            }))
                            .into()
                        }
                        "expr" => {
                            let expr = self.expression().unwrap();
                            return EmitFn(Box::new(move |compiler| expr.to_bytecode(compiler)))
                                .into();
                        }
                        "pop" => {
                            return EmitFn(Box::new(|compiler| {
                                compiler.bytecode.function.chunk.emit_op(OpCode::Pop);
                            }))
                            .into()
                        }
                        "debug_stack" => {
                            return EmitFn(Box::new(|compiler| {
                                compiler
                                    .bytecode
                                    .function
                                    .chunk
                                    .emit_op(OpCode::CallNative(idx_debug_stack!() as u16))
                            }))
                            .into()
                        }
                        "assert_stack" => {
                            self.consume(TokenKind::LeftBracket, "expected left bracket to close");
                            let mut exprs = Vec::new();
                            loop {
                                if self.match_token(TokenKind::RightBracket) {
                                    break;
                                }
                                exprs.push(self.expression().unwrap());
                                if !self.match_token(TokenKind::Comma) {
                                    self.advance();
                                    break;
                                }
                            }
                            return EmitFn(Box::new(move |compiler| {
                                exprs.iter().for_each(|expr| expr.to_bytecode(compiler));
                                compiler.bytecode.write_call_fn_arg_ptr_op(
                                    idx_assert_stack!() as u8,
                                    exprs.len() as u8,
                                );
                            }))
                            .into();
                        }
                        _ => return self.node(),
                    }
                }
                panic!()
            }
            _ => self.statement(),
        };
        if self.panic_mode {
            println!("synchronizing");
            self.synchronize();
        }
        node
    }
    pub fn expression_statement(&mut self) -> Node {
        let expr = self.expression().unwrap().to_expr();
        self.consume(
            TokenKind::SemiColon,
            format!("Expected ';' after expression {}:{}", file!(), line!()).as_str(),
        );
        Statement::Expression(expr).to_node()
    }
    pub fn token_as_identifier(&mut self) -> Identifier {
        self.advance();
        Identifier {
            value: self.previous().clone(),
        }
    }
    pub fn statement(&mut self) -> Node {
        match self.current().kind {
            TokenKind::If => {
                self.advance();
                self.if_expr(false)
            }
            TokenKind::While => {
                self.advance();
                self.while_expr(false)
            }
            TokenKind::LeftBrace => {
                self.advance();
                self.block(false)
            }
            TokenKind::Print => {
                self.advance();
                let node = Statement::Print(Box::new(self.expression().unwrap())).to_node();
                self.consume(TokenKind::SemiColon, "Expected ';' ");
                node
            }
            TokenKind::AssertEq => {
                self.advance();
                let lhs = self.expression().unwrap().to_expr();
                self.consume(TokenKind::Comma, "Expected ','' to seperate lhs and rhs");
                let rhs = self.expression().unwrap().to_expr();
                self.consume(TokenKind::SemiColon, "Expected ';'");

                let node = Statement::AssertEq(lhs, rhs);
                node.to_node()
            }
            TokenKind::AssertNe => {
                self.advance();
                let lhs = self.expression().unwrap().to_expr();
                self.consume(TokenKind::Comma, "Expected ','' to seperate lhs and rhs");
                let rhs = self.expression().unwrap().to_expr();
                self.consume(TokenKind::SemiColon, "Expected ';'");

                let node = Statement::AssertNe(lhs, rhs);
                node.to_node()
            }
            TokenKind::Let => {
                self.advance();
                let identifier = self.token_as_identifier();
                if !self.check(TokenKind::Equal) {
                    self.consume(TokenKind::SemiColon, "Expected 'n' after variable declaration");
                    return VariableDeclaration {
                        intializer: Expression::None ,
                        identifier,
                    }.to_node()
                }
                self.consume(TokenKind::Equal, "Expected '=' after variable name");
                let initializer = self.expression().unwrap().to_expr();
                self.consume(
                    TokenKind::SemiColon,
                    "Expected ';' after variable declaration",
                );

                VariableDeclaration {
                    intializer: initializer,
                    identifier,
                }
                .to_node()
            }
            TokenKind::Func => {
                self.advance();
                let identifier = self.token_as_identifier();
                let mut parameters: Vec<Identifier> = Vec::new();
                self.consume(TokenKind::LeftParen, "err");
                loop {
                    if self.match_token(TokenKind::RightParen) {
                        break;
                    }
                    parameters.push(self.expression().unwrap().as_identifier());
                    if !self.match_token(TokenKind::Comma) {
                        self.advance();
                        break;
                    }
                }
                self.consume(TokenKind::LeftBrace, "Expected '{'");
                FunctionDeclaration {
                    parameters,
                    name: identifier,
                    block: self.block(false).to_expr().as_block(),
                }
                .to_declaration()
                .to_node()
            }
            TokenKind::Return => {
                self.advance();
                if self.scope_depth == 0 && self.function_type.eq(&FunctionType::Script) {
                    self.error("Cannot return from the top level of a script")
                }
                if let Ok(expr) = self.expression() {
                    let expr = expr.to_expr();
                    self.consume(
                        TokenKind::SemiColon,
                        format!("Expected ';' after expression {}:{}", file!(), line!()).as_str(),
                    );
                    Statement::Return(ReturnStmt { expr: Some(expr) }).to_node()
                } else {
                    self.consume(TokenKind::SemiColon, "Expected ';' after return statement");
                    Statement::Return(ReturnStmt { expr: None }).to_node()
                }
            }
            _ => self.expression_statement(),
        }
    }
}
impl Parser<'_> {
    pub fn identifier(&mut self, can_assign: bool) -> Node {
        let token = self.previous().clone();
        let is_global = self.scope_depth == 0;
        if can_assign && self.match_token(TokenKind::Equal) {
            return BinaryExpr {
                lhs: Box::new(Identifier { value: token }.to_node()),
                op: self.previous().clone(),
                rhs: Box::new(self.expression().unwrap()),
            }
            .to_expr()
            .to_node();
        }
        Identifier { value: token }.to_node()
    }
    pub fn call_expr(&mut self, lhs: Node) -> Node {
        let expr = lhs.to_expr();
        let mut parameters: Vec<Expression> = Vec::new();
        loop {
            if self.match_token(TokenKind::RightParen) {
                break;
            }

            let parameter = self.expression();
            parameters.push(parameter.unwrap().to_expr());
            if !self.match_token(TokenKind::Comma) {
                self.advance();
                break;
            }
        }
        Call {
            parameters: Box::new(parameters),
            expr: Box::new(expr),
        }
        .to_expr()
        .to_node()
    }
    pub fn while_expr(&mut self, _can_assign: bool) -> Node {
        let condition = self.expression().unwrap().to_expr();
        let block = self.expression().unwrap().to_expr().as_block();

        While {
            predicate: Box::new(condition),
            block,
        }
        .to_expr()
        .to_node()
    }
    pub fn if_expr(&mut self, _can_assign: bool) -> Node {
        let condition = self.expression().unwrap().to_expr();
        let then = self.expression().unwrap().to_expr().as_block();
        #[allow(unused_mut)]
        let mut else_block = None;
        if self.match_token(TokenKind::Else) {
            let block = self.expression().unwrap().to_expr().as_block();
            else_block = Some(block)
        }
        If {
            predicate: Box::new(condition),
            then,
            else_block,
        }
        .to_expr()
        .to_node()
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
        block.to_node()
    }
    pub fn string(&mut self, _can_assign: bool) -> Node {
        Literal(
            Literals::String(self.previous().lexeme.clone()),
            self.previous().clone(),
        )
        .as_node()
    }
    pub fn binary(&mut self, lhs: Node) -> Node {
        let rule = Self::get_rule(self.previous().kind);
        let op = self.previous().clone();
        // the precedence is +1 so it'll compile it as the rhs
        #[allow(unsafe_code)]
        let prec: Precedence = unsafe { transmute((rule.precedence as u8) + 1) };
        let rhs = self.precedence(prec).unwrap();

        Expression::Binary(BinaryExpr {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
        })
        .to_node()
    }
    pub fn number(&mut self, _can_assign: bool) -> Node {
        let token = self.previous().clone();
        let number = token.lexeme.parse::<f64>().unwrap();
        Literal(Literals::Number(number), token).as_node()
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
macro_rules! error_at_current {
    ($parser:expr, $msg:expr) => {{
        $parser.had_error = true;
        let current = $parser.current().to_owned();
        error_at!($parser, &current, $msg);
    }};
}

macro_rules! error_at {
    ($parser:expr, $token:expr, $msg:expr) => {{
        $parser.panic_mode = true;
        let mut diagnostics = $parser.diagnostics.borrow_mut();

        match $token.kind {
            TokenKind::EOF => {
                diagnostics.log(
                    Some(&$token.position),
                    "Compiler",
                    "Error at EOF: ".to_string(),
                );
            }

            _ => {
                let range: Range<usize> = ($token.position.start_in_source as usize)
                    ..($token.position.start_in_source as usize + $token.length);
                diagnostics.log(
                    Some(&$token.position),
                    "Compiler",
                    format!("Error at `{}`: ", &$parser.scanner.source[range]),
                );
            }
        }
        println!("{}", $msg.red());
        println!(
            "{}",
            format!("(location:  {}:{})", file!(), line!()).black()
        );
    }};
}
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
        error_at!(self, &previous, msg);
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
            println!("{}", self.previous().kind);
            match self.previous().kind {
                TokenKind::Return
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Func
                | TokenKind::Let => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }

    pub fn new(
        scanner: Scanner,
        diagnostics: Rc<RefCell<Diagnostics<'a>>>,
        function_type: FunctionType,
    ) -> Parser<'a> {
        Parser {
            diagnostics,
            scanner,
            had_error: false,
            panic_mode: false,
            scope_depth: 0,
            function_type,
            token_state: TokenState {
                current: EOF.to_owned(),
                previous: Rc::new(EOF.to_owned()),
                tokens: Vec::new(),
                index: 0,
            },
        }
    }
    /// if the current token matches the token kind, then advance, if not return false
    pub fn match_token(&mut self, tk: TokenKind) -> bool {
        if !self.check(tk) {
            return false;
        };
        self.advance();
        true
    }
    /// checks the token kind of the current
    pub fn check(&mut self, tk: TokenKind) -> bool {
        self.current().kind == tk
    }
    pub fn previous(&self) -> &Token {
        &self.token_state.previous
    }
    pub fn current(&self) -> &Token {
        &self.token_state.current
    }

    pub fn consume(&mut self, kind: TokenKind, err: &str) -> &Token {
        let current = self.current().kind;
        if current.ne(&kind) {
            error_at_current!(self, err);
            return self.current();
        }

        return self.advance();
    }

    pub fn peek(&mut self, distance: usize) -> &Token {
        &self.scanner.tokens[self.scanner.tokens.len() - (1 + distance)]
    }
}

impl<'a> Parser<'a> {
    pub fn advance(&mut self) -> &Token {
        let current = self.scanner.next_token();
        self.token_state
            .tokens
            .insert(self.token_state.index, self.token_state.current.to_owned());
        self.token_state.previous =
            Rc::new(self.token_state.tokens[self.token_state.index].to_owned());
        self.token_state.current = current;
        self.token_state.index += 1;
        &self.token_state.current
    }
}
