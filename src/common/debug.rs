use std::fmt::Display;

use super::{chunk::Chunk, opcode::OpCode};

pub fn dissasemble_chunk(chunk: &Chunk) {
    let mut instruction_ptr: usize = 0;

    loop {
        if instruction_ptr >= chunk.code.len() {
            break;
        }
        let instruction = &chunk.code[instruction_ptr];
        print!("\t");
        match instruction {
            OpCode::DefineGlobal(pos)
            | OpCode::Constant(pos)
            | OpCode::SetGlobal(pos)
            | OpCode::GetGlobal(pos) => {
                let constant = &chunk.constants[*pos as usize];

                println!("{} {}", instruction.to_string(), constant)
            }
            _ => println!("{}", instruction.to_string()),
        }
        instruction_ptr += 1;
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Constant(pos)
            | OpCode::DefineGlobal(pos)
            | OpCode::GetGlobal(pos)
            | OpCode::SetGlobal(pos) => {
                write!(f, "{:?}", self)
            }
            x => write!(f, "{:?}", x),
        }
    }
}
