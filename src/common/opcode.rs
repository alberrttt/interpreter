use super::value::Value;
use macros::ExpandOpCode;
use strum::Display;

pub type ConstantLocation = u16;
pub type SlotIndex = u8;
pub type Offset = usize;

#[repr(u8)]
#[derive(Debug, Display, Clone, PartialEq, ExpandOpCode)]

pub enum OpCode {
    Equal,
    NotEqual,
    Constant(ConstantLocation),
    GetLocal(u16),
    DefineLocal(ConstantLocation),
    SetLocal(u16),
    /// this doesn't clone off the stack, and uses mem::take instead of cloning
    SetLocalConsumes(u16),
    DefineGlobal(ConstantLocation),
    GetGlobal(ConstantLocation),
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
    Pop,
    Print,
    Add,
    Sub,
    Div,
    Mul,
    Return,
    Nop,
    CallNative(u16),
    CallNativeArgPtr(u16, *const [Value]),
}
