use std::default;

use super::value::Value;
use crate::frontend::bytecode::Bytecode;
use macros::ExpandOpCode;
use strum::Display;

pub type ConstantLocation = u16;
pub type SlotIndex = u8;
pub type Offset = usize;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StackInfo {
    pub push: u8,
    pub pop: u8,
}

#[repr(u8)]
#[derive(Debug, Display, Clone, PartialEq, Default, ExpandOpCode)]
pub enum OpCode {
    #[stack(pop = 2, push = 1)]
    Equal,
    #[stack(pop = 2, push = 1)]
    NotEqual,
    #[stack(push = 1)]
    Constant(ConstantLocation),
    GetLocal(u16),
    #[stack(push = 1)]
    DefineLocal(ConstantLocation),
    #[stack(push = 1)]
    SetLocal(u16),
    /// this doesn't clone off the stack, and uses mem::take instead of cloning
    #[stack(pop = 1)]
    SetLocalConsumes(u16),
    #[stack(pop = 1)]
    DefineGlobal(ConstantLocation),
    #[stack(push = 1)]
    GetGlobal(ConstantLocation),
    #[stack(pop = 1, push = 0)]
    SetGlobal(ConstantLocation),
    Exit,
    PopJumpToIfFalse(Offset),
    JumpToIfFalse(Offset),
    JumpTo(Offset),
    Call(usize),
    Greater,
    Less,
    GreaterEq,
    LessEq,
    AssertEq,
    AssertNe,
    True,
    False,
    Void,
    Not,
    Negate,
    #[stack(pop = 1)]
    Pop,
    #[stack(pop = 1)]
    Print,
    Add,
    Sub,
    Div,
    Mul,
    Return,
    #[default]
    Nop,
    CallNative(u16),
    CallNativeArgPtr(u16, *const [Value]),
}
fn __test() {
    let mut b = Bytecode::default();
    b.write_add_op()
}
