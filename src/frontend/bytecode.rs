use crate::common::{function::Function, opcode::OpCode};

use super::compiler::{
    local::{Local, LOCAL},
    FunctionType,
};
#[derive(Debug, Clone)]
pub struct Bytecode {
    pub function: Function,
    pub scope_depth: u8,
    pub locals: [Local; 512],
    pub local_count: usize,
    pub emit_after_block: Vec<OpCode>,
    pub function_type: FunctionType,

    pub compiling_statement: bool,

    pub returned_from_block: bool,
}

impl Default for Bytecode {
    fn default() -> Self {
        Self {
            function: Default::default(),
            scope_depth: Default::default(),
            locals: [LOCAL; 512],
            local_count: Default::default(),
            emit_after_block: Default::default(),
            function_type: Default::default(),
            compiling_statement: Default::default(),
            returned_from_block: Default::default(),
        }
    }
}

impl Bytecode {}
