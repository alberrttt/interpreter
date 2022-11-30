use strum::Display;
#[derive(Debug, Display)]
pub enum OpCode {
    Constant(u16),
    // use string interning ? maybe
    // u16 is the location in the constant pool
    DefineGlobal(u16),
    GetGlobal(u16),
    SetGlobal(u16),
    True,
    False,

    Pop,
    Print,
    Add,
    Sub,
    Div,
    Mul,
    Return,
}
