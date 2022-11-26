// runtime value

use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(Ptr<String>),
}
pub trait AsValue {
    fn as_value(self) -> Value;
}
impl AsValue for String {
    fn as_value(self) -> Value {
        Value::String(rcrf(self))
    }
}
impl AsValue for f64 {
    fn as_value(self) -> Value {
        Value::Number(self)
    }
}
pub type Ptr<T> = Rc<RefCell<T>>;

pub fn rcrf<T>(inner: T) -> Ptr<T> {
    Rc::new(RefCell::new(inner))
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::String(string) => {
                write!(f, "{}", *string.borrow())
            }
        }
    }
}
