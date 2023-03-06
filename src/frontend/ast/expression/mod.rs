use crate::{
    common::opcode::OpCode,
    frontend::{
        compiler::Compiler,
        error::ParseError,
        error::{ParseResult, SyntaxError},
        parser::{Parse, Parser, Rule},
        scanner::TokenKind,
        Precedence,
    },
};

use self::{
    binary_expr::BinaryExpr, block::Block, call_expr::Call, if_expr::If, while_expr::While,
};

use super::{
    identifier::Identifier,
    literal::{Literal, Literals},
    node::{AsNode, EmitFn, Node},
    CompileToBytecode,
};
pub trait AsExpr {
    fn to_expr(self) -> Expression;
}
pub mod binary_expr;
pub mod block;
pub mod call_expr;
pub mod if_expr;
pub mod while_expr;
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Grouping(Box<Expression>),
    Binary(BinaryExpr),
    Literal(Literal),
    Not(Box<Expression>),
    Negate(Box<Expression>),
    Block(Block),
    Identifier(Identifier),
    If(If),
    While(While),
    CallExpr(Call),
    None,
}
impl Parse<Node> for Expression {
    fn parse(parser: &mut Parser) -> crate::frontend::error::ParseResult<Node> {
        Self::parse_precedence(parser, Precedence::None)
    }
}
impl From<Expression> for Node {
    fn from(value: Expression) -> Self {
        Node::Expression(value)
    }
}
impl AsNode for Expression {
    fn to_node(self) -> Node {
        Node::Expression(self)
    }
}
impl Expression {
    pub fn get_rule<'a>(kind: TokenKind) -> Rule<'a> {
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
                    parser
                        .consume(TokenKind::RightParen, "expected right parenthesis to close")
                        .expect("msg");
                    expr
                }),
                infix: Some(Parser::call_expr),
            },
            TokenKind::While => Rule {
                precedence: Precedence::None,
                prefix: Some(Parser::while_expr),
                infix: None,
            },
            TokenKind::Equal => Rule {
                precedence: Precedence::Assignment,
                prefix: None,
                infix: Some(Parser::binary),
            },
            TokenKind::Greater
            | TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual
            | TokenKind::EqualEqual
            | TokenKind::BangEqual => Rule {
                precedence: Precedence::Comparison,
                prefix: None,
                infix: Some(Parser::binary),
            },
            TokenKind::If => Rule {
                precedence: Precedence::None,
                prefix: Some(Parser::if_expr),
                infix: None,
            },
            TokenKind::LeftBrace => Rule {
                precedence: Precedence::None,
                prefix: Some(|parser, _can_assign| parser.block(_can_assign).into()),
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
                prefix: Some(|parser, can_assign| {
                    Parser::identifier(parser, can_assign).expect("expected identifier")
                }),
                infix: None,
            },
            TokenKind::Number => Rule {
                precedence: Precedence::None,
                infix: None,
                prefix: Some(Parser::number),
            },
            TokenKind::Star | TokenKind::Slash => Rule {
                precedence: Precedence::Factor,
                prefix: None,
                infix: Some(Parser::binary),
            },
            TokenKind::Plus => Rule {
                infix: Some(Parser::binary),
                prefix: None,
                precedence: Precedence::Term,
            },
            TokenKind::Dash => Rule {
                infix: Some(Parser::binary),
                prefix: Some(|parser, _can_assign| {
                    Expression::Negate(Box::new(
                        Self::parse_precedence(parser, Precedence::Unary)
                            .unwrap()
                            .to_expr(),
                    ))
                    .to_node()
                }),
                precedence: Precedence::Term,
            },
            TokenKind::Bang => Rule {
                precedence: Precedence::Unary,
                prefix: Some(|parser, _can_assign| {
                    Expression::Not(Box::new(
                        Self::parse_precedence(parser, Precedence::Unary)
                            .unwrap()
                            .to_expr(),
                    ))
                    .to_node()
                }),
                infix: None,
            },
            TokenKind::String => Rule {
                precedence: Precedence::None,
                prefix: Some(Parser::string),

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
    pub fn parse_precedence(parser: &mut Parser, prec: Precedence) -> ParseResult<Node> {
        parser.advance();
        let _path = {
            let binding = parser.diagnostics.borrow();
            binding.file_path.to_str().unwrap()
        };
        let previous = parser.previous();
        let rule = Self::get_rule(previous.kind);
        let can_assign: bool = prec <= Precedence::Assignment;
        #[allow(unused_assignments)]
        let mut expression: Node = Node::None;
        if rule.prefix.is_some() {
            expression = rule.prefix.unwrap()(parser, can_assign);
        } else {
            return Err(ParseError::SyntaxError(SyntaxError(
                vec![parser.previous().clone()],
                "expected expression".to_string(),
            )));
        }

        loop {
            if parser.at_end() {
                break Ok(expression);
            }
            let current = parser.current();
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

            parser.advance();
            let previous = parser.previous();

            match Self::get_rule(previous.kind).infix {
                None => {}
                Some(infix) => {
                    expression = infix(parser, expression);
                }
            }
        }
    }

    pub fn as_literal(self) -> Literal {
        let Expression::Literal(literal) = self else {panic!()};
        literal
    }
}
impl Expression {
    pub fn as_block(self) -> Block {
        let Expression::Block(block) = self else {
            panic!()
        };

        block
    }
    pub fn as_binary_expr(self) -> BinaryExpr {
        let Expression::Binary(expr) = self else {
            panic!()
        };
        expr
    }
}
impl CompileToBytecode for Expression {
    fn to_bytecode(&self, compiler: &mut Compiler) {
        match self {
            Expression::None => {
                // fix later
                compiler.bytecode.write_void_op()
            }
            Expression::CallExpr(call_expr) => call_expr.to_bytecode(compiler),
            Expression::While(while_expr) => while_expr.to_bytecode(compiler),
            Expression::Grouping(inner) => inner.to_bytecode(compiler),
            Expression::Literal(literal) => literal.to_bytecode(compiler),
            Expression::Not(expr) => {
                expr.to_bytecode(compiler);
                compiler.bytecode.function.chunk.emit_op(OpCode::Not);
            }
            Expression::If(if_expr) => if_expr.to_bytecode(compiler),
            Expression::Negate(expr) => {
                expr.to_bytecode(compiler);
                compiler.bytecode.function.chunk.emit_op(OpCode::Negate);
            }
            Expression::Block(block) => block.to_bytecode(compiler),
            Expression::Identifier(identifier) => identifier.to_bytecode(compiler),
            super::Expression::Binary(binary) => binary.to_bytecode(compiler),
        }
    }
}
