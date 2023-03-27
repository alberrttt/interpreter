use std::rc::Rc;

use crate::{
    common::{
        opcode::OpCode,
        value::{AsValue, Value},
    },
    frontend::{
        ast::{expression::block::Block, identifier::Identifier, node::Node, CompileToBytecode},
        compiler::{Compiler, Enclosing, FunctionType},
        parser::Parse,
        scanner::TokenKind,
        statement::return_stmt::ReturnStmt,
        typesystem::Primitive,
    },
};

use super::Declaration;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub block: Block,
    pub return_type: Option<Primitive>,
    pub parameters: Vec<Parameter>,
}
impl Parse<FunctionDeclaration> for FunctionDeclaration {
    fn parse(
        parser: &mut crate::frontend::parser::Parser,
    ) -> crate::frontend::error::ParseResult<FunctionDeclaration>
    where
        FunctionDeclaration: Into<crate::frontend::node::Node>,
    {
        parser.advance();
        let identifier = parser.token_as_identifier();
        let mut parameters: Vec<Parameter> = Vec::new();
        parser.consume(TokenKind::LeftParen, "err").unwrap();
        loop {
            if parser.match_token(TokenKind::RightParen) {
                break;
            }
            parameters.push(Parameter::parse(parser)?);
            if !parser.match_token(TokenKind::Comma) {
                parser.advance();
                break;
            }
        }
        let mut return_type: Option<Primitive> = None;
        if parser.match_token(TokenKind::RightArrow) {
            let primitive: Primitive = parser.expression().unwrap().as_identifier().into();
            return_type = Some(primitive);
        }
        parser.consume(TokenKind::LeftBrace, "Expected '{'")?;

        Ok(FunctionDeclaration {
            name: identifier,
            parameters,
            block: parser.block(false),
            return_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Identifier,
    pub type_annotation: Option<Identifier>,
}
impl Parse<Parameter> for Parameter {
    fn parse(
        parser: &mut crate::frontend::parser::Parser,
    ) -> crate::frontend::error::ParseResult<Parameter>
    where
        Parameter: Into<crate::frontend::node::Node>,
    {
        let identifier = parser.token_as_identifier();
        let mut primitive: Option<Identifier> = if parser.match_token(TokenKind::Colon) {
            Some(parser.expression()?.as_identifier())
        } else {
            None
        };
        // reuse this

        Ok(Parameter {
            name: identifier,
            type_annotation: primitive,
        })
    }
}
impl From<Parameter> for Node {
    fn from(value: Parameter) -> Self {
        panic!()
    }
}
impl From<FunctionDeclaration> for Declaration {
    fn from(value: FunctionDeclaration) -> Self {
        Declaration::FunctionDeclaration(value)
    }
}
impl From<FunctionDeclaration> for Node {
    fn from(value: FunctionDeclaration) -> Self {
        Node::Declaration(value.into())
    }
}
impl<'a> Compiler<'a> {
    fn in_scope(&self) -> bool {
        self.bytecode.scope_depth > 0
    }
}
impl CompileToBytecode for FunctionDeclaration {
    fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        // uses the current compiler's compilation context for the function
        // which is returned later
        compiler
            .bytecode
            .scope
            .last_mut()
            .unwrap()
            .insert(self.name.value.lexeme.clone(), self.clone().into());
        let lexeme = self.name.value.lexeme.clone();
        let _name = compiler
            .bytecode
            .function
            .chunk
            .emit_value(lexeme.to_value());
        compiler.bytecode.globals.push(lexeme);
        let mut temp_compiler = Compiler::new(compiler.diagnostics.clone(), FunctionType::Function);
        temp_compiler.enclosing = Some(Enclosing(compiler));
        temp_compiler.bytecode.return_type = self.return_type.clone();
        temp_compiler
            .bytecode
            .globals
            .extend(compiler.bytecode.globals.clone());
        let function = {
            // sets the function name and arity
            temp_compiler.bytecode.function.arity = self.parameters.len() as u8;
            temp_compiler.bytecode.function.name = self.name.value.lexeme.clone();

            // parameters need to be added to be scope and local array
            for param in &self.parameters {
                temp_compiler.add_local(param.name.value.clone());
                let scope = temp_compiler.bytecode.scope.last_mut().unwrap();
                scope.insert(
                    param.name.value.lexeme.to_owned(),
                    Primitive::from(param.type_annotation.to_owned().unwrap()).into(),
                );
            }

            // finally compiles the block
            // typechecking will happen in `<ReturnStmt as CompileToBytecode>::to_bytecode`

            self.block.to_bytecode(&mut temp_compiler);

            if self.return_type.is_none() {
                temp_compiler.bytecode.write_void_op();
                temp_compiler
                    .bytecode
                    .function
                    .chunk
                    .emit_op(OpCode::Return);
            }
            temp_compiler.bytecode.function
        };
        let function = Rc::new(function);
        let location = compiler
            .bytecode
            .function
            .chunk
            .emit_value(Value::Function(function.clone()));
        compiler.bytecode.write_closure_op(location);

        let count = function.upvalue_count;
        let upvalues = &temp_compiler.bytecode.upvalues[..count].to_vec();
        for upvalue in upvalues {
            compiler.bytecode.write_byte(upvalue.is_local as u8);
            compiler.bytecode.write_byte(upvalue.index)
        }
        if compiler.in_scope() {
            compiler.add_local(self.name.value.clone());
        } else {
            // location of the name in the constant pool
            let lexeme = self.name.value.lexeme.clone();
            let name = compiler
                .bytecode
                .function
                .chunk
                .emit_value(lexeme.to_value());
            compiler.bytecode.globals.push(lexeme);
            compiler
                .bytecode
                .function
                .chunk
                .emit_op(OpCode::DefineGlobal(name))
        };
        // compilation context is returned
    }
}
