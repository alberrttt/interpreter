use std::default;

use super::value::Value;
use crate::frontend::{ast::statement, bytecode::Bytecode};
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
// generate attributes for each opcode
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
    #[stack(pop = 1)]
    PopJumpToIfFalse(Offset),
    JumpToIfFalse(Offset),
    JumpTo(Offset),

    Call(usize),
    #[stack(pop = 2, push = 1)]
    Greater,
    #[stack(pop = 2, push = 1)]
    Less,
    #[stack(pop = 2, push = 1)]
    GreaterEq,
    #[stack(pop = 2, push = 1)]
    LessEq,
    #[stack(pop = 2)]
    AssertEq,
    #[stack(pop = 2)]
    AssertNe,
    #[stack(push = 1)]
    True,
    #[stack(push = 1)]
    False,
    #[stack(push = 1)]
    Void,
    #[stack(pop = 1, push = 1)]
    Not,
    #[stack(pop = 1, push = 1)]
    Negate,
    #[stack(pop = 1)]
    Pop,
    #[stack(pop = 1)]
    Print,
    #[stack(pop = 2, push = 1)]
    Add,
    #[stack(pop = 2, push = 1)]
    Sub,
    #[stack(pop = 2, push = 1)]
    Div,
    #[stack(pop = 2, push = 1)]
    Mul,
    #[stack(pop = 1, push = 1)]
    Return,
    #[default]
    Nop,
    CallNative(u16),
    CallFnArgPtr(u8, u8),
}
