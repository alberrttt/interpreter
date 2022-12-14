use super::{chunk::Chunk, opcode::OpCode};

use std::string::ToString;

pub fn dissasemble_chunk(chunk: &Chunk) {
    println!("----------------------");
    let mut instruction_ptr: usize = 0;

    loop {
        if instruction_ptr >= chunk.code.len() {
            break;
        }
        let instruction = &chunk.code[instruction_ptr];
        print!("{instruction_ptr} \t");
        instruction_ptr = diassasemble_instruction(instruction_ptr, instruction, chunk);
    }
    println!("----------------------");
}

pub fn diassasemble_instruction(
    instruction_ptr: usize,
    instruction: &OpCode,
    chunk: &Chunk,
) -> usize {
    match instruction {
        OpCode::DefineGlobal(pos)
        | OpCode::Constant(pos)
        | OpCode::SetGlobal(pos)
        | OpCode::GetGlobal(pos)
        | OpCode::DefineLocal(pos) => {
            let constant = &chunk.constants[*pos as usize];

            println!("{} <{}>", instruction.to_string(), constant)
        }
        OpCode::JumpTo(offset) | OpCode::JumpToIfFalse(offset) | OpCode::Call(offset) => {
            println!("{} {}", instruction.to_string(), offset)
        }
        OpCode::GetLocal(pos) | OpCode::SetLocal(pos) => {
            println!("{} {}", instruction.to_string(), pos)
        }
        OpCode::SetTempSlot(pos) | OpCode::TakeTempSlot(pos) => {
            println!("{} {}", instruction.to_string(), pos)
        }
        _ => println!("{}", instruction.to_string()),
    }
    instruction_ptr + 1
}
