use std::rc::Rc;

use crate::common::function::Function;

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub function: Option<Rc<Function>>,
    pub ip: usize,
    pub slots: usize,
}

impl CallFrame {
    pub fn new(function: Function) -> CallFrame {
        let callframe = CallFrame {
            function: Some(Rc::new(function)),
            ip: 0,
            slots: 0,
        };
        callframe
    }
}
