// runtime value

use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ptr::addr_of,
    rc::Rc,
};

use super::function::Function;

#[repr(u8)]
#[derive(Clone, Default)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(Ptr<String>),
    Function(Ptr<Function>),
    Array(Ptr<Vec<Value>>),
    Void,
    #[default]
    None,
}
impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array(arg0) => f.debug_tuple("Array").field(arg0).finish(),
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Boolean(arg0) => f.debug_tuple("Boolean").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Function(_arg0) => f.debug_tuple("Function").finish(),
            Self::Void => write!(f, "Void"),
            Self::None => write!(f, "None"),
        }
    }
}
pub const NONEVALUE: Value = Value::None;
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
    fn to_value(self) -> Value;
}
impl AsValue for bool {
    fn to_value(self) -> Value {
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
            unreachable!()
        }
    }
}
impl AsValue for String {
    fn to_value(self) -> Value {
        Value::String(rcrf(self))
    }
}
impl AsValue for f64 {
    fn to_value(self) -> Value {
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
                let tmp = string.as_ref().borrow();
                write!(f, "{}", tmp)
            }
            Value::Boolean(bool) => {
                write!(f, "{}", bool)
            }
            Value::None | Value::Void => {
                panic!("cannot print a void or none");
            }
            Value::Function(function) => {
                write!(f, "<func {:?}>", addr_of!(function))
            }
            Value::Array(array) => {
                let tmp = array.as_ref().borrow();
                write!(f, "{:?}", tmp)
            }
        }
    }
}
