use crate::{
    cli_context::{self, Context},
    common::{function::Function, opcode::OpCode},
};

use super::{ast::CompileToBytecode, parser::Parser, scanner::Scanner};

pub struct Compiler<'a> {
    pub function: Function,
    pub scanner: Scanner,
    pub parser: Parser,
    pub context: &'a mut Context<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new(context: &'a mut Context<'a>) -> Compiler<'a> {
        Compiler {
            function: Function::new(),
            scanner: Scanner::new(String::from("")),
            parser: Parser::new(vec![]),
            context,
        }
    }
    pub fn compile(&mut self, source: String) -> Function {
        let mut scanner = Scanner::new(source);
        scanner.scan_thru();
        let mut parser = Parser::new(scanner.tokens);
        let mut parsed = parser.parse_file();
        let mut function = parsed.build_function();
        function.chunk.emit_many(vec![OpCode::Return]);
        function
    }
}
