use crate::common::{
    function::BytecodeFunction,
    opcode::{OpCode, StackInfo},
};

use super::{
    ast::expression::Expression,
    compiler::{
        local::{Local, LOCAL},
        FunctionType,
    },
};
#[derive(Debug, Clone)]
pub struct Bytecode {
    pub stack_info: Vec<StackInfo>,
    pub function: BytecodeFunction,
    pub scope_depth: u8,
    pub locals: Box<[Local]>,
    pub local_count: usize,
    pub emit_after_block: Vec<OpCode>,
    pub function_type: FunctionType,
    pub compiling_statement: bool,
    pub current_expr: Option<*const Expression>,
    pub returned_from_block: bool,
    pub eliminated: bool,
}

impl Default for Bytecode {
    fn default() -> Self {
        Self {
            stack_info: Vec::new(),
            function: Default::default(),
            scope_depth: Default::default(),
            locals: Box::new([LOCAL; 512]),
            local_count: Default::default(),
            emit_after_block: Default::default(),
            function_type: Default::default(),
            compiling_statement: Default::default(),
            returned_from_block: Default::default(),
            current_expr: Default::default(),
            eliminated: Default::default(),
        }
    }
}

impl Bytecode {
    pub fn start_expr(&mut self, expr: &Expression) {
        self.current_expr = Some(expr);
    }
}
