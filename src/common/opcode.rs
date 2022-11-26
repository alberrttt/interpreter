#[derive(Debug)]
pub enum OpCode {
    Constant(u16),
    // use string interning ? maybe
    // u16 is the location in the constant pool
    GetGlobal(u16),
    SetGlobal(u16),
    Pop,
    Print,
    Add,
    Sub,
    Div,
    Mul,
    Return,
}
