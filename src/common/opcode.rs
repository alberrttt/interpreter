use strum::Display;

use super::value::Value;

pub type ConstantLocation = u16;
pub type SlotIndex = u8;
pub type Offset = usize;

#[repr(u8)]
#[derive(Debug, Display, Clone, PartialEq)]

pub enum OpCode {
    Equal,
    NotEqual,
    Constant(ConstantLocation),
    // use string interning ? maybe
    // u16 is the location in the constant pool
    GetLocal(u16),
    DefineLocal(ConstantLocation),
    SetLocal(u16),
    DefineGlobal(ConstantLocation),
    GetGlobal(ConstantLocation),
    SetGlobal(ConstantLocation),
    Exit,
    PopJumpToIfFalse(Offset),
    JumpToIfFalse(Offset),
    JumpTo(Offset),
    // Call(arguments)
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
