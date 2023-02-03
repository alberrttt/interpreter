use std::rc::Rc;

use super::function::Function;

#[derive(Debug, Clone)]
pub struct Closure {
    pub func: Rc<Function>,
}

impl From<&Rc<Function>> for Closure {
    fn from(value: &Rc<Function>) -> Self {
        Closure {
            func: value.clone(),
        }
    }
}
