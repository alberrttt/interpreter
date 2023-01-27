use crate::common::function::BytecodeFunction;

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub function: *const BytecodeFunction,
    pub ip: usize,
    pub slots: usize,
}

impl CallFrame {
    pub fn new(function: &BytecodeFunction) -> CallFrame {
        CallFrame {
            function,
            ip: 0,
            slots: 0,
        }
    }
}
