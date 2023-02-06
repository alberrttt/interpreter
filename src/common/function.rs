use super::chunk::Chunk;
#[derive(Debug, Clone, Default)]
pub struct Function {
    pub chunk: Chunk,
    pub arity: u8,
    pub name: String,
    pub upvalue_count: usize,
}

impl Function {
    pub fn new() -> Function {
        Function {
            chunk: Chunk::new(),
            arity: 0,
            name: String::from("main"),
            upvalue_count: 0,
        }
    }
}
