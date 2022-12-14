use std::fmt::Debug;

use crate::backend::vm::VirtualMachine;

use super::value::Value;

pub struct Native(pub fn(args: &[Value], vm: &mut VirtualMachine));
impl Debug for Native {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
