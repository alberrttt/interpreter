use std::fmt::Debug;

use macros::native_macro;
use phf::phf_map;
use strum::Display;

use crate::backend::vm::{natives, VirtualMachine};

use super::value::Value;
#[repr(u8)]
#[derive(Display, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Natives {
    Str,
    Num,
}

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

native_macro! {
    ident => Native(),
    hello => Native()
}
