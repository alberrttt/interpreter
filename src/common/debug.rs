use std::borrow::Borrow;

use crate::common::value::Value;

use super::{chunk::Chunk, opcode::OpCode};

pub fn dissasemble_chunk(chunk: &Chunk, name: &str) {
    chunk
        .constants
        .iter()
        .filter_map(|v| {
            if let Value::Function(v) = v {
                Some(v)
            } else {
                None
            }
        })
        .for_each(|f| {
            let function = &f.as_ref().borrow();
            dissasemble_chunk(&function.chunk, &function.name)
        });
    println!("{name} ----------------------");
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

            println!("{} <{:?}>", instruction, constant)
        }

        OpCode::JumpTo(offset)
        | OpCode::JumpToIfFalse(offset)
        | OpCode::PopJumpToIfFalse(offset)
        | OpCode::Call(offset) => {
            println!("{} {}", instruction, offset)
        }
        OpCode::GetLocal(pos) | OpCode::SetLocal(pos) => {
            println!("{} {}", instruction, pos)
        }

        _ => println!("{}", instruction),
    }
    instruction_ptr + 1
}
