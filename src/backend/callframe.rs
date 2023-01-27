use crate::common::function::Function;

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub function: *const Function,
    pub ip: usize,
    pub slots: usize,
}

impl CallFrame {
    pub fn new(function: &Function) -> CallFrame {
        CallFrame {
            function,
            ip: 0,
            slots: 0,
        }
    }
}
