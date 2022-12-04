use std::ptr::null;

use crate::{
    cli_context::Context,
    common::{function::Function, opcode::OpCode},
};

use super::{
    ast::CompileToBytecode,
    parser::{Parser},
    scanner::{Position, Scanner, Token, TokenKind},
};

#[derive(Debug)]
pub struct Enclosing<'a>(*mut Compiler<'a>);
impl<'a> Enclosing<'a> {
    pub fn get_compiler(&self) -> &Compiler<'a> {
        unsafe { self.0.as_ref().unwrap() }
    }
    pub fn get_compiler_mut(self) -> &'a mut Compiler<'a> {
        unsafe { self.0.as_mut().unwrap() }
    }
}
#[derive(Debug)]
pub struct Compiler<'a> {
    pub function: Function,
    pub scanner: Scanner,
    pub parser: Parser<'a>,
    pub context: Option<&'a mut Context<'a>>,
    pub scope_depth: u8,
    pub locals: [Local; 512],
    pub local_count: usize,
    pub enclosing: Option<Enclosing<'a>>,
    pub emit_after_block: Vec<OpCode>,
    pub function_type: FunctionType,
}
#[derive(Debug, PartialEq)]
pub enum FunctionType {
    Script, // file
    Function,
}
#[derive(Debug, Default, Clone)]
pub struct Local {
    pub name: Token,
    pub depth: u8,
}
impl Local {
    pub fn new() -> Local {
        Local {
            name: Token::default(),
            depth: 0,
        }
    }
}
const LOCAL: Local = Local {
    name: Token {
        kind: TokenKind::Error,
        value: String::new(),
        line: 9999,
        length: 9999,
        position: Position {
            start_in_line: 9999,
            start_in_source: 9999,
            line: 9999,
        },
    },
    depth: 0,
};
#[derive(Debug)]
pub enum CompileResult {
    Error,
}
impl<'a> Compiler<'a> {
    pub fn new(context: &'a mut Context<'a>, function_type: FunctionType) -> Compiler<'a> {
        let compiler = Compiler {
            function: Function::new(),
            scanner: Scanner::new(String::from("")),
            parser: Parser::new(Box::new(Scanner::new("".to_string())), None, null()),
            context: Some(context),
            locals: [LOCAL; 512],
            local_count: 0,
            scope_depth: 0,
            enclosing: None,
            emit_after_block: Vec::new(),
            function_type,
        };
        compiler
    }

    pub fn compile(mut self, source: String) -> Result<Function, CompileResult> {
        let scanner = Box::new(Scanner::new(source));

        let parser = Parser::new(scanner, Some(self.context.take().unwrap()), &self);
        self.parser = parser;

        let parsed = self.parser.parse_file();
        if self.parser.had_error {
            return Err(CompileResult::Error);
        }
        self.context = self.parser.context.take();
        let function = Function::new();
        self.function = function;
        for node in parsed.nodes {
            node.to_bytecode(&mut self)
        }
        self.function.chunk.emit_many(vec![OpCode::Return]);
        Ok(self.function)
    }
}
