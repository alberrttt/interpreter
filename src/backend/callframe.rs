use crate::common::{
    closure::{self, Closure},
    function::Function,
};

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub closure: *const Closure,
    pub ip: usize,
    pub slots: usize,
}

impl CallFrame {
    pub fn new(closure: &Closure) -> CallFrame {
        CallFrame {
            closure,
            ip: 0,
            slots: 0,
        }
    }
}
