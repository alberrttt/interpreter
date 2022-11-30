use crate::{
    cli_context::Context,
    common::{function::Function, opcode::OpCode},
};

use super::{
    ast::CompileToBytecode,
    parser::Parser,
    scanner::{Scanner, Token, TokenKind},
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
    pub enclosing: Option<Enclosing<'a>>,
}
#[derive(Debug, Default, Clone)]
pub struct Local {
    name: Token,
    depth: u8,
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
        start: 9999,
        line_start: 9999,
    },
    depth: 0,
};
impl<'a> Compiler<'a> {
    pub fn new(context: &'a mut Context<'a>) -> Compiler<'a> {
        let parser = Parser::new(vec![], None);
        Compiler {
            function: Function::new(),
            scanner: Scanner::new(String::from("")),
            parser,
            context: Some(context),
            locals: [LOCAL; 512],
            scope_depth: 0,
            enclosing: None,
        }
    }
    pub fn compile(mut self, source: String) -> Function {
        let scanner = Scanner::new(source);
        self.scanner = scanner;

        self.scanner.scan_thru();
        let parser = Parser::new(
            self.scanner.tokens.clone(),
            Some(self.context.take().unwrap()),
        );
        self.parser = parser;

        let parsed = self.parser.parse_file();
        self.context = self.parser.context.take();
        let function = Function::new();
        self.function = function;
        for node in parsed.nodes {
            node.to_bytecode(&mut self)
        }
        self.function.chunk.emit_many(vec![OpCode::Return]);
        self.function
    }
}
