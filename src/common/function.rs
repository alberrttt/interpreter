use super::chunk::Chunk;
#[derive(Debug, Clone, Default)]
pub struct BytecodeFunction {
    pub chunk: Chunk,
    pub arity: u8,
    pub name: String,
}

impl BytecodeFunction {
    pub fn new() -> BytecodeFunction {
        BytecodeFunction {
            chunk: Chunk::new(),
            arity: 0,
            name: String::from("main"),
        }
    }
}
