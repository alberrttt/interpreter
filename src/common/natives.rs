use std::fmt::Debug;

use phf::phf_map;
use strum::Display;

use crate::backend::vm::VirtualMachine;

use super::value::Value;
#[repr(u8)]
#[derive(Display, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Natives {
    Str,
    Num,
}

pub static NATIVES: phf::Map<Natives, Native> = phf_map! {};
pub struct Native(pub fn(args: &mut Vec<Value>, vm: &VirtualMachine));
impl Debug for Native {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl From<Natives> for usize {
    fn from(native: Natives) -> Self {
        native as usize
    }
}
