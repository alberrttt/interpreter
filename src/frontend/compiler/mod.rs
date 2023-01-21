pub mod local;
use local::{Local, LOCAL};

use std::{cell::RefCell, rc::Rc};

/// its so messy omg..
use crate::{
    cli_helper::Diagnostics,
    common::{chunk::Chunk, function::Function, interner::StringInterner, opcode::OpCode},
};

use super::{
    ast::CompileToBytecode,
    bytecode::{self, Bytecode},
    parser::Parser,
    scanner::Scanner,
};

#[derive(Debug)]
pub struct Enclosing<'a>(*mut Compiler<'a>);
impl<'a> Enclosing<'a> {
    pub fn get_compiler(&self) -> &Compiler<'a> {
        #[allow(unsafe_code)]
        unsafe {
            self.0.as_ref().unwrap()
        }
    }
}
#[derive(Debug)]
pub struct Compiler<'a> {
    pub scanner: Scanner,
    pub parser: Parser<'a>,
    pub enclosing: Option<Enclosing<'a>>,
    pub diagnostics: Rc<RefCell<Diagnostics<'a>>>,
    pub interner: Rc<RefCell<StringInterner>>,
    pub bytecode: Bytecode,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum FunctionType {
    #[default]
    Script, // file
    Function,
}

#[derive(Debug)]
pub enum CompileResult {
    Error,
}
impl<'a> Compiler<'a> {
    pub fn new(
        interner: Rc<RefCell<StringInterner>>,
        diagnostics: Rc<RefCell<Diagnostics<'a>>>,
        function_type: FunctionType,
    ) -> Compiler<'a> {
        Compiler {
            scanner: Scanner::default(),
            parser: Parser::default(),
            enclosing: None,
            diagnostics,
            interner,
            bytecode: Bytecode::default(),
        }
    }

    pub fn compile(mut self, source: String) -> Result<Function, CompileResult> {
        let scanner = Scanner::new(source);

        let parser = Parser::new(
            scanner,
            self.diagnostics.clone(),
            self.bytecode.function_type.clone(),
        );
        self.parser = parser;

        let parsed_file = self.parser.parse_file();
        if self.parser.had_error {
            return Err(CompileResult::Error);
        }
        let function = Function::new();
        self.bytecode.function = function;
        parsed_file.to_bytecode(&mut self);
        self.bytecode.write_return_op();
        dbg!(self.bytecode.stack_info);
        Ok(self.bytecode.function)
    }
}
