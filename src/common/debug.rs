use super::{chunk::Chunk, opcode::OpCode};

use std::string::ToString;

pub fn dissasemble_chunk(chunk: &Chunk) {
    let mut instruction_ptr: usize = 0;

    loop {
        if instruction_ptr >= chunk.code.len() {
            break;
        }
        let instruction = &chunk.code[instruction_ptr];
        print!("{instruction_ptr} \t");
        match instruction {
            OpCode::DefineGlobal(pos)
            | OpCode::Constant(pos)
            | OpCode::SetGlobal(pos)
            | OpCode::GetGlobal(pos)
            | OpCode::DefineLocal(pos) => {
                let constant = &chunk.constants[*pos as usize];

                println!("{} <{}>", instruction.to_string(), constant)
            }
            OpCode::Jump(offset) | OpCode::JumpIfFalse(offset) => {
                println!(
                    "{} {}",
                    instruction.to_string(),
                    instruction_ptr + *offset as usize + 1
                )
            }
            OpCode::GetLocal(pos) | OpCode::SetLocal(pos) => {
                println!("{} {}", instruction.to_string(), pos)
            }
            OpCode::SetTempSlot(pos) | OpCode::TakeTempSlot(pos) => {
                println!("{} {}", instruction.to_string(), pos)
            }
            _ => println!("{}", instruction.to_string()),
        }
        instruction_ptr += 1;
    }
}
