use super::{opcode::OpCode, value::Value};
#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: vec![],
            constants: vec![],
        }
    }
    pub fn emit_op(&mut self, op: OpCode) {
        self.code.push(op)
    }
    pub fn emit_many(&mut self, mut ops: Vec<OpCode>) {
        self.code.append(&mut ops);
    }
    pub fn emit_constant(&mut self, value: Value) -> u16 {
        let pos = self.emit_value(value);
        self.emit_op(OpCode::Constant(pos));
        pos
    }
    pub fn emit_value(&mut self, value: Value) -> u16 {
        let pos = self.constants.len();
        self.constants.push(value);
        return pos as u16;
    }
}
