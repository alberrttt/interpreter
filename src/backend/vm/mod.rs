use std::{collections::HashMap, time::Instant};

use crate::common::{
    chunk::Chunk,
    function::Function,
    interner::StringInterner,
    natives::Native,
    opcode::OpCode,
    value::{AsValue, Value},
};

use super::callframe::CallFrame;

pub mod natives;
pub mod ops;
pub const FUNCTION: Function = Function {
    chunk: Chunk {
        code: Vec::new(),
        constants: Vec::new(),
    },
    arity: 0,
    name: String::new(),
};

#[derive(Debug)]
pub struct VirtualMachine {
    pub stack: Vec<Value>,
    pub callframes: [CallFrame; 2048],
    pub frame_count: usize,
    pub globals: HashMap<usize, Value>,
    pub natives: Vec<Native>,
    pub interner: StringInterner,
}

impl VirtualMachine {
    pub fn new(interner: StringInterner) -> VirtualMachine {
        pub const CALLFRAME: CallFrame = CallFrame {
            function: std::ptr::null(),
            ip: 0,
            slots: 0,
        };
        VirtualMachine {
            callframes: [CALLFRAME; 2048],
            stack: vec![],
            natives: vec![(Native(|_: &[Value], vm: _| println!("stack dump: {:?}", vm.stack)))],
            globals: HashMap::new(),
            frame_count: 0,
            interner,
        }
    }
    #[allow(clippy::not_unsafe_ptr_arg_deref, unsafe_code)]
    pub fn call(&mut self, function: *const Function, arg_count: usize) {
        let arity = unsafe { (*function).arity };
        if arg_count != arity as usize {
            panic!(
                "mismatched argument counts! expected {} got {arg_count}",
                arity
            )
        }

        let frame = &mut self.callframes[self.frame_count];
        frame.function = function;
        frame.slots = self.stack.len() - (arg_count + 1);

        self.frame_count += 1;
    }
    pub fn run(mut self) {
        let start = Instant::now();
        let mut current_frame = &self.callframes[self.frame_count - 1];
        macro_rules! read_current_frame_fn {
            () => {{
                #[allow(unsafe_code)]
                unsafe {
                    &(*current_frame.function)
                }
            }};
        }
        let mut function = read_current_frame_fn!();
        let mut chunk = &function.chunk;
        let mut ip: usize = current_frame.ip;
        let _interner = &self.interner;

        macro_rules! pop {
            () => {{
                assert!(self.stack.len() > 0);

                let i = self.stack.len() - 1;
                let tmp = ::std::mem::take(&mut self.stack[i]);
                #[allow(unsafe_code)]
                unsafe {
                    self.stack.set_len(i);
                }
                if tmp.eq(&Value::Void) {
                    panic!()
                }
                tmp
            }};
        }
        macro_rules! binary_op {
            ($op:tt) => {{
                let rhs = pop!();
                let lhs = pop!();

                match lhs {
                    Value::Number(lhs) => {
                        let Value::Number(rhs) = rhs else {
                            panic!()
                        };
                        self.stack.push(Value::Number(lhs $op rhs))
                    }
                    x => unimplemented!("cannot apply binary operation to value {}", x),
                }
            }};
        }
        macro_rules! binary_op_bool {
            ($op:tt) => {{
                let rhs = pop!();
                let lhs = pop!();

                match lhs {
                    Value::Number(lhs) => {
                        let Value::Number(rhs) = rhs else {
                            panic!()
                        };
                        self.stack.push(Value::Boolean(lhs $op rhs))
                    }
                    x => unimplemented!("cannot apply binary operation to value {}", x),
                }
            }};
        }
        loop {
            let instruction = &chunk.code[ip];
            // #[cfg(debug_assertions)]
            // {
            // print!("{ip} Executing ");
            // diassasemble_instruction(ip, instruction, &function.chunk);
            // }
            ip += 1;

            match instruction.clone() {
                OpCode::SetLocalConsumes(index) => {
                    self.stack[index as usize + 1 + current_frame.slots] = {
                        let tmp = self.stack.len() - 1;
                        std::mem::take(&mut self.stack[tmp])
                    };
                }
                OpCode::Equal => {
                    binary_op_bool!(==)
                }
                OpCode::NotEqual => {
                    binary_op_bool!(!=)
                }
                OpCode::CallNativeArgPtr(_, _) => {
                    todo!();
                    // let native = &self.natives[location as usize];
                    // let args = unsafe { &*ptr };
                    // native.0(args, &self)
                }
                OpCode::CallNative(location) => {
                    let native = &self.natives[location as usize];
                    let args = [];
                    (native.0)(&args, &self);
                }
                OpCode::JumpTo(offset) => {
                    ip = offset;
                }
                OpCode::PopJumpToIfFalse(offset) => {
                    let popped = pop!();
                    let condition = popped.as_bool();
                    if !condition {
                        ip = offset;
                    }
                }

                OpCode::JumpToIfFalse(offset) => {
                    let condition = self.stack[self.stack.len() - 1].as_bool();
                    if !condition {
                        ip = offset;
                    }
                }
                OpCode::Nop => {}
                OpCode::Not => {
                    let pop = pop!();
                    if let Value::Boolean(bool) = pop {
                        self.stack.push((!bool).to_value());
                    } else {
                        panic!("not cannot be applied to {} ", pop)
                    }
                }
                OpCode::Negate => {
                    let pop = pop!();
                    if let Value::Number(num) = pop {
                        self.stack.push((-num).to_value());
                    } else {
                        panic!("negate cannot be applied to {} ", pop)
                    }
                }
                OpCode::True => self.stack.push(Value::Boolean(true)),
                OpCode::False => self.stack.push(Value::Boolean(false)),
                OpCode::Constant(location) => {
                    self.stack.push(chunk.constants[location as usize].clone())
                }
                OpCode::GetLocal(index) => {
                    let value = self.stack[index as usize + 1 + current_frame.slots].clone();
                    self.stack.push(value)
                }
                OpCode::SetLocal(index) => {
                    self.stack[index as usize + 1 + current_frame.slots] =
                        self.stack.last().unwrap().clone();
                }
                OpCode::DefineLocal(location) => {
                    self.stack.push(chunk.constants[location as usize].clone())
                }
                OpCode::GetGlobal(name) => {
                    let name = chunk.constants[name as usize].as_string();
                    self.stack.push(self.globals.get(&name.0).unwrap().clone())
                }
                OpCode::SetGlobal(name) => {
                    let name = chunk.constants[name as usize].as_string();
                    assert!(self.globals.contains_key(&name.0));
                    let value = self.stack[self.stack.len() - 1].clone();
                    self.globals.insert(name.0, value);
                }
                OpCode::DefineGlobal(name) => {
                    let name = chunk.constants[name as usize].as_string();
                    let value = pop!();
                    self.globals.insert(name.0, value);
                }
                OpCode::Void => self.stack.push(Value::Void),
                OpCode::Add => {
                    let rhs = pop!();
                    let lhs = pop!();

                    match lhs {
                        Value::Number(lhs) => {
                            let Value::Number(rhs) = rhs else {
                                panic!()
                            };
                            self.stack.push(Value::Number(lhs + rhs))
                        }
                        Value::String(lhs) => {
                            let Value::String(rhs) = rhs else {
                                panic!("lhs {:?}\nrhs{:?}",lhs,rhs);
                            };

                            let mut lhs: String = lhs.into();
                            let rhs: String = rhs.into();
                            lhs.push_str(rhs.as_str());
                            self.stack.push(lhs.to_value());
                        }
                        _ => unimplemented!(),
                    }
                }
                OpCode::Sub => {
                    binary_op!(-)
                }
                OpCode::Mul => {
                    binary_op!(*)
                }
                OpCode::Pop => {
                    #[allow(unsafe_code)]
                    unsafe {
                        self.stack.set_len(self.stack.len() - 1);
                    }
                }
                OpCode::Div => {
                    binary_op!(/)
                }
                OpCode::Print => {
                    println!("{}", pop!());
                }
                OpCode::AssertEq => {
                    let rhs = pop!();
                    let lhs = pop!();

                    assert_eq!(lhs, rhs);
                }
                OpCode::AssertNe => {
                    let rhs = pop!();
                    let lhs = pop!();

                    assert_ne!(lhs, rhs);
                }
                OpCode::Exit => break,
                OpCode::Return => {
                    let returning = pop!();
                    self.frame_count -= 1;

                    if self.frame_count == 0 {
                        println!("vm took {}", start.elapsed().as_secs_f64());
                        return;
                    }

                    current_frame = &self.callframes[self.frame_count - 1];
                    function = read_current_frame_fn!();
                    chunk = &function.chunk;
                    ip = current_frame.ip;
                    self.stack.truncate(self.callframes[self.frame_count].slots);
                    self.stack.push(returning);
                }
                OpCode::Call(arg_count) => {
                    let callee = &self.stack[self.stack.len() - (1 + arg_count)];
                    let Value::Function(callee) = callee else {
                        panic!()
                    };
                    let callee: *const Function = callee.as_ptr() as *const _;

                    self.call(callee, arg_count);
                    self.callframes[self.frame_count - 2].ip = ip;

                    // prepares for the next callframe
                    {
                        current_frame = &self.callframes[self.frame_count - 1];
                        function = read_current_frame_fn!();
                        chunk = &function.chunk;
                        ip = 0;
                    }
                }

                OpCode::Less => {
                    binary_op_bool!(<)
                }
                OpCode::LessEq => {
                    binary_op_bool!(<=)
                }
                OpCode::Greater => {
                    binary_op_bool!(>)
                }
                OpCode::GreaterEq => {
                    binary_op_bool!(>=)
                }
            }
        }
    }
}
