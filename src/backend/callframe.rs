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
