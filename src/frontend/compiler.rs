use crate::{
    cli_context::Context,
    common::{function::Function, opcode::OpCode},
};

use super::{ast::CompileToBytecode, parser::Parser, scanner::Scanner};

#[derive(Debug)]
pub struct Compiler<'a> {
    pub function: Function,
    pub scanner: Scanner,
    pub parser: Parser<'a>,
    pub context: Option<&'a mut Context<'a>>,
}

impl<'a> Compiler<'a> {
    pub fn new(context: &'a mut Context<'a>) -> Compiler<'a> {
        let parser = Parser::new(vec![], None);
        Compiler {
            function: Function::new(),
            scanner: Scanner::new(String::from("")),
            parser,
            context: Some(context),
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
