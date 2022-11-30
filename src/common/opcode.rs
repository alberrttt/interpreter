use strum::Display;
#[derive(Debug, Display)]
pub enum OpCode {
    Constant(u16),
    // use string interning ? maybe
    // u16 is the location in the constant pool
    DefineGlobal(u16),
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
}
