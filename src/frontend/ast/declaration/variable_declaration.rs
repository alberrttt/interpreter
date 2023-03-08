use crate::{
    common::{opcode::OpCode, value::AsValue},
    frontend::{
        ast::{
            expression::Expression,
            identifier::Identifier,
            node::{AsNode, Node},
            CompileToBytecode,
        },
        compiler::{local::Local, Compiler},
        parser::Parse,
        scanner::{Token, TokenKind},
        types::{Primitive, Signature},
    },
};

use super::{AsDeclaration, Declaration};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub identifier: Identifier,
    pub intializer: Expression,
    // pub mutable: bool,
}

impl Parse<Self> for VariableDeclaration {
    fn parse(
        parser: &mut crate::frontend::parser::Parser,
    ) -> crate::frontend::error::ParseResult<Self> {
        parser.advance();
        let identifier = parser.token_as_identifier();
        if !parser.check(TokenKind::Equal) {
            parser.consume(
                TokenKind::SemiColon,
                "Expected 'n' after variable declaration",
            )?;
            return Ok(VariableDeclaration {
                intializer: Expression::None,
                identifier,
            });
        }
        parser.consume(TokenKind::Equal, "Expected '=' after variable name")?;
        let initializer = parser.expression().unwrap().into();
        parser.consume(
            TokenKind::SemiColon,
            "Expected ';' after variable declaration",
        )?;

        Ok(VariableDeclaration {
            intializer: initializer,
            identifier,
        })
    }
}
impl From<VariableDeclaration> for Declaration {
    fn from(value: VariableDeclaration) -> Self {
        Declaration::VariableDeclaration(value)
    }
}
impl From<VariableDeclaration> for Node {
    fn from(value: VariableDeclaration) -> Self {
        Node::Declaration(value.into())
    }
}

impl CompileToBytecode for VariableDeclaration {
    fn to_bytecode(&self, compiler: &mut Compiler) {
        self.intializer.to_bytecode(compiler);
        if compiler.bytecode.scope_depth > 0 {
            compiler.add_local(self.identifier.value.clone());
            return;
        }
        let function = &mut compiler.bytecode.function;
        let lexeme = self.identifier.value.lexeme.clone();
        let name = function.chunk.emit_value(lexeme.to_value());
        compiler
            .bytecode
            .scope
            .last_mut()
            .unwrap()
            .insert(lexeme.clone(), {
                let initializer = self.intializer.clone();
                let typed: Primitive = match initializer {
                    Expression::Literal(lit) => lit.into(),
                    Expression::None => panic!(),
                    _ => panic!(),
                };

                Signature::Variable(Box::new(typed))
            });
        compiler.bytecode.globals.push(lexeme);
        function.chunk.emit_op(OpCode::DefineGlobal(name))
    }
}
impl<'a> Compiler<'a> {
    pub fn add_local(&mut self, name: Token) {
        self.bytecode.locals.push(Local {
            name,
            depth: self.bytecode.scope_depth,
            is_captured: false,
        });
        self.bytecode.local_count += 1;
    }
}
impl AsDeclaration for VariableDeclaration {
    fn to_declaration(self) -> super::Declaration {
        super::Declaration::VariableDeclaration(self)
    }
}
impl AsNode for VariableDeclaration {
    fn to_node(self) -> crate::frontend::ast::node::Node {
        Node::Declaration(self.to_declaration())
    }
}
