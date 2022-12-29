use std::{cell::RefCell, rc::Rc};

/// its so messy omg..
use crate::{
    cli_helper::Context,
    common::{function::Function, interner::StringInterner, opcode::OpCode},
};

use super::{
    ast::CompileToBytecode,
    parser::Parser,
    scanner::{Position, Scanner, Token, TokenKind},
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
    pub context: Rc<RefCell<Context<'a>>>,
    pub interner: Rc<RefCell<StringInterner>>,

    pub function: Function,
    pub scope_depth: u8,
    pub locals: [Local; 512],
    pub local_count: usize,
    pub emit_after_block: Vec<OpCode>,
    pub function_type: FunctionType,

    pub compiling_statement: bool,

    pub returned_from_block: bool,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum FunctionType {
    #[default]
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
        lexeme: String::new(),
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
    pub fn new(
        interner: Rc<RefCell<StringInterner>>,
        context: Rc<RefCell<Context<'a>>>,
        function_type: FunctionType,
    ) -> Compiler<'a> {
        Compiler {
            context,
            locals: [LOCAL; 512],
            local_count: 0,
            scope_depth: 0,
            enclosing: None,
            emit_after_block: Vec::new(),
            function_type,
            returned_from_block: false,

            compiling_statement: false,

            scanner: Scanner::default(),
            function: Function::default(),
            parser: Parser::default(),
            interner,
        }
    }

    pub fn compile(mut self, source: String) -> Result<Function, CompileResult> {
        let scanner = Scanner::new(source);

        let parser = Parser::new(scanner, self.context.clone(), self.function_type.clone());
        self.parser = parser;

        let parsed_file = self.parser.parse_file();
        if self.parser.had_error {
            return Err(CompileResult::Error);
        }
        let function = Function::new();
        self.function = function;
        parsed_file.to_bytecode(&mut self);

        self.function.chunk.emit_many(vec![OpCode::Return]);

        Ok(self.function)
    }
}
