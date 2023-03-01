use crate::{
    backend::callframe::CallFrame,
    common::{chunk::Chunk, closure::Closure, function::Function, opcode::OpCode, value::Value},
};

use super::VirtualMachine;

impl VirtualMachine {
    #[inline(always)]
    pub fn closure_op(&mut self, env: (&Chunk, &Function, usize, &CallFrame), location: u16) {
        let (chunk, _function, mut ip, current_callframe) = env;
        let function = &chunk.constants[location as usize];

        let Value::Function(function) = function else {
            panic!("{function:?}")
        };
        let mut closure: Closure = function.into();
        for x in 0..function.upvalue_count {
            let OpCode::Byte(is_local) = &chunk.code[ip] else {
                panic!()
            };
            ip += 1;
            let OpCode::Byte(index) = &chunk.code[ip] else {
                panic!()
            };
            ip += 1;

            let is_local = *is_local != 0;
            let index = *index;

            if is_local {
                self.capture_upvalue(index.into(), &mut closure, current_callframe)
            } else {
                closure.upvalues[x] = closure.upvalues[index as usize].clone()
            }
        }

        self.stack.push(Value::Closure(Box::new(closure)))
    }
}
