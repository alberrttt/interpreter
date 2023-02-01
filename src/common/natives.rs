use std::fmt::Debug;

use macros::native_macro;
use phf::phf_map;
use strum::Display;

use crate::backend::vm::VirtualMachine;

use super::value::{AsValue, Value};

pub struct Native(pub fn(vm: &mut VirtualMachine, args: Vec<Value>));
impl Debug for Native {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
