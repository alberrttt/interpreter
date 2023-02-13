use std::mem;

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
        .for_each(|function| dissasemble_chunk(&function.chunk, &function.name));
    println!("{name} ----------------------");
    let mut instruction_ptr: usize = 0;

    loop {
        if instruction_ptr >= chunk.code.len() {
            break;
        }
        let instruction = &chunk.code[instruction_ptr];
        print!("{instruction_ptr:0>4} \t");

        instruction_ptr = diassasemble_instruction(instruction_ptr, instruction, chunk);
    }
    println!("----------------------");
}

pub fn diassasemble_instruction(
    mut instruction_ptr: usize,
    instruction: &OpCode,
    chunk: &Chunk,
) -> usize {
    instruction_ptr += 1;

    match instruction {
        OpCode::DefineGlobal(pos)
        | OpCode::Constant(pos)
        | OpCode::SetGlobal(pos)
        | OpCode::GetGlobal(pos)
        | OpCode::DefineLocal(pos) => {
            let constant = &chunk.constants[*pos as usize];

            println!("{instruction} <{constant:?}>")
        }
        OpCode::GetUpValue(pos) | OpCode::SetUpValue(pos) => {
            println!("{instruction} <{pos}>")
        }
        OpCode::CallFnArgPtr(_, args) => {
            println!("{instruction} {args}")
        }
        OpCode::CallNative(location) => {
            println!("{instruction} <idx:{location}>")
        }
        OpCode::Closure(closure) => {
            let constant = &chunk.constants[(*closure as usize)];
            let Value::Function(function) = constant else {
                dbg!(&chunk.constants);
                dbg!(closure);
                panic!()
            };
            println!("{instruction} <{constant:?}> <name:{}>", function.name);
            for _ in 0..function.upvalue_count {
                let byte = chunk.code[instruction_ptr].clone();
                let is_local = if let OpCode::Byte(byte) = byte {
                    byte
                } else {
                    panic!("{byte} {instruction_ptr}")
                } != 0;
                instruction_ptr += 1;
                let index = chunk.code[instruction_ptr].clone();
                instruction_ptr += 1;
                let OpCode::Byte(index) = index else {
                    panic!()
                };

                println!(
                    "{:0>4}\t|\t\t{} {}",
                    instruction_ptr - 2,
                    if is_local { "local" } else { "upvalue" },
                    index
                );
            }
        }
        OpCode::JumpTo(offset)
        | OpCode::JumpToIfFalse(offset)
        | OpCode::PopJumpToIfFalse(offset)
        | OpCode::Call(offset) => {
            println!("{instruction} {offset}")
        }
        OpCode::GetLocal(pos) | OpCode::SetLocal(pos) => {
            println!("{instruction} {pos}")
        }

        _ => println!("{}", instruction),
    }
    instruction_ptr
}
