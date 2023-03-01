use crate::common::closure::Closure;

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub closure: *mut Closure,
    pub ip: usize,
    pub slots: usize,
}
