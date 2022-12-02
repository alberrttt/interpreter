use strum::Display;

pub type ConstantLocation = u16;
#[derive(Debug, Display)]
pub enum OpCode {
    Constant(ConstantLocation),
    // use string interning ? maybe
    // u16 is the location in the constant pool
    GetLocal(u16),
    DefineLocal(ConstantLocation),
    SetLocal(u16),

    DefineGlobal(ConstantLocation),
    GetGlobal(u16),
    SetGlobal(u16),
    AssertEq,
    True,
    False,
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
}
