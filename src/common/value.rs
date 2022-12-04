// runtime value

use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(Ptr<String>),
    Void,
    None,
}
pub const NoneValue: Value = Value::None;
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0.borrow_mut().eq(&*r0.as_ref().borrow()),
            _ => false,
        }
    }
}
pub trait AsValue {
    fn as_value(self) -> Value;
}
impl AsValue for bool {
    fn as_value(self) -> Value {
        Value::Boolean(self)
    }
}
impl Value {
    pub fn as_bool(&self) -> &bool {
        let Value::Boolean(bool) = &self else {
            panic!()
        };
        bool
    }
    pub fn as_string(&self) -> &String {
        if let Value::String(string) = self {
            unsafe { &*string.as_ref().as_ptr() }
        } else {
            panic!()
        }
    }
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
            Value::Boolean(bool) => {
                write!(f, "{}", bool)
            }
            Value::None | Value::Void => {
                panic!("cannot print a void or none");
            }
        }
    }
}
