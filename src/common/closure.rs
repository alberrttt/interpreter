use std::rc::Rc;

use super::{function::Function, value::RuntimeUpvalue};

#[derive(Debug, Clone)]
pub struct Closure {
    pub func: Rc<Function>,
    pub upvalues: Vec<RuntimeUpvalue>,
}

impl From<&Rc<Function>> for Closure {
    fn from(value: &Rc<Function>) -> Self {
        Closure {
            func: value.clone(),
            upvalues: Vec::new(),
        }
    }
}
