// runtime value

use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ptr::addr_of,
    rc::Rc,
};

use crate::frontend::bytecode::Upvalue;

use super::{
    closure::Closure,
    function::Function,
    interner::{InternedString, STRING_INTERNER},
};

#[repr(u8)]
#[derive(Clone, Default)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(InternedString),
    Function(Rc<Function>),
    Array(Ptr<Vec<Value>>),
    Closure(Box<Closure>),
    Void,
    #[default]
    None,
}
#[derive(Debug, Clone)]
pub struct RuntimeUpvalue {
    pub location: Rc<RefCell<Value>>, // maybe this needs to be a pointer
    pub index: u8,
}
impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array(arg0) => f.debug_tuple("Array").field(arg0).finish(),
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Boolean(arg0) => f.debug_tuple("Boolean").field(arg0).finish(),
            Self::String(arg0) => {
                let mut tuple = f.debug_tuple("String");
                tuple.field(arg0);
                let string: String = (*arg0).into();
                tuple.field(&string);
                tuple.finish()
            }
            Self::Function(_arg0) => f.debug_tuple("Function").finish(),
            Self::Void => write!(f, "Void"),
            Self::None => write!(f, "None"),
            Self::Closure(closure) => write!(f, "<closure {:?}>", closure.func),
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0.eq(r0),
            (Self::Void, Self::Void) => true,
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}
pub trait AsValue {
    fn to_value(self) -> Value;
}
impl AsValue for bool {
    #[inline(always)]
    fn to_value(self) -> Value {
        Value::Boolean(self)
    }
}
impl Value {
    #[inline(always)]
    pub fn as_bool(&self) -> &bool {
        let Value::Boolean(bool) = &self else {
            panic!()
        };
        bool
    }
    #[inline(always)]
    pub fn as_string(&self) -> &InternedString {
        if let Value::String(string) = self {
            string
        } else {
            unreachable!()
        }
    }
}
impl AsValue for &str {
    #[inline(always)]
    fn to_value(self) -> Value {
        let mut interner = STRING_INTERNER.lock().expect("already?");
        Value::String(interner.get_or_intern(self))
    }
}
impl AsValue for f64 {
    #[inline(always)]
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
                let tmp: String = (*string).into();
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
            Value::Closure(function) => {
                write!(f, "<closure {:?}>", addr_of!(function))
            }
            Value::Array(array) => {
                let tmp = array.as_ref().borrow();
                write!(f, "{:?}", tmp)
            }
        }
    }
}
