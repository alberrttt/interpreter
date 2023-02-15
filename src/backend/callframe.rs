use crate::common::{
    closure::{self, Closure},
    function::Function,
};

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub closure: *mut Closure,
    pub ip: usize,
    pub slots: usize,
}
