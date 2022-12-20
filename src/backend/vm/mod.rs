use std::{
    cell::RefCell,
    collections::HashMap,
    mem::MaybeUninit,
    rc::Rc,
    thread::sleep_ms,
    time::{Duration, Instant},
};

use crate::common::{
    chunk::Chunk,
    debug::diassasemble_instruction,
    function::Function,
    natives::Native,
    opcode::OpCode,
    value::{AsValue, Value},
};

use super::callframe::CallFrame;

pub mod ops;

pub const FUNCTION: Function = Function {
    chunk: Chunk {
        code: Vec::new(),
        constants: Vec::new(),
    },
    arity: 0,
    name: String::new(),
};
pub const CALLFRAME: CallFrame = CallFrame {
    function: FUNCTION,
    ip: 0,
    slots: 0,
};
#[derive(Debug)]
pub struct VirtualMachine {
    pub stack: Vec<Value>,
    pub callframes: [CallFrame; 2048],
    pub frame_count: usize,
    pub globals: HashMap<String, Value>,
    pub natives: Vec<Native>,
}
impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            callframes: [CALLFRAME; 2048],
            stack: vec![],
            natives: vec![(Native(|_: &[Value], vm: _| println!("stack dump: {:?}", vm.stack)))],
            globals: HashMap::new(),
            frame_count: 0,
        }
    }
    pub fn call(&mut self, function: Function, arg_count: usize) {
        if arg_count != function.arity as usize {
            panic!(
                "mismatched argument counts! expected {} got {arg_count}",
                function.arity
            )
        }

        let frame = &mut self.callframes[self.frame_count];
        frame.function = function;
        frame.slots = self.stack.len() - (arg_count + 1) as usize;

        self.frame_count += 1;
    }
    pub fn run(mut self) {
        let start = Instant::now();
        let mut current_frame = &self.callframes[self.frame_count - 1].clone();
        let mut function = &current_frame.function;
        let mut chunk = &function.chunk;
        let mut ip: usize = current_frame.ip;
        loop {
            let instruction = &chunk.code[ip as usize];
            // #[cfg(debug_assertions)]
            // {
            //     print!("{ip} Executing ");
            //     diassasemble_instruction(ip, instruction, &function.chunk);
            // }
            ip += 1;

            match instruction.clone() {
                OpCode::CallNativeArgPtr(location, ptr) => {
                    let native = &self.natives[location as usize];
                    let args = unsafe { &*ptr };
                    native.0(&args, &self)
                }
                OpCode::CallNative(location) => {
                    let native = &self.natives[location as usize];
                    let args = [];
                    (native.0)(&args, &self);
                }
                OpCode::JumpTo(offset) => {
                    ip = offset as usize;
                }
                OpCode::PopJumpToIfFalse(offset) => {
                    let popped = self.stack.pop().unwrap();
                    let condition = popped.as_bool();
                    if !condition {
                        ip = offset as usize;
                    }
                }

                OpCode::JumpToIfFalse(offset) => {
                    let condition = self.stack[self.stack.len() - 1].as_bool();
                    if !condition {
                        ip = offset as usize;
                    }
                }
                OpCode::Nop => {}
                OpCode::Not => {
                    let pop = self.stack.pop().unwrap();
                    if let Value::Boolean(bool) = pop {
                        self.stack.push((!bool).as_value());
                    } else {
                        panic!("not cannot be applied to {} ", pop)
                    }
                }
                OpCode::Negate => {
                    let pop = self.stack.pop().unwrap();
                    if let Value::Number(num) = pop {
                        self.stack.push((-num).as_value());
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
                    self.stack.push(self.globals.get(name).unwrap().clone())
                }
                OpCode::SetGlobal(name) => {
                    let name = (chunk.constants[name as usize].as_string()).to_owned();
                    assert!(self.globals.contains_key(&name));
                    let value = self.stack[self.stack.len() - 1].clone();
                    self.globals.insert(name, value);
                }
                OpCode::DefineGlobal(name) => {
                    let name = (chunk.constants[name as usize].as_string()).to_owned();
                    let value = self.stack.pop().unwrap();
                    self.globals.insert(name, value);
                }
                OpCode::Void => self.stack.push(Value::Void),
                OpCode::Add => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();

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

                            let mut lhs = lhs.borrow().to_owned();
                            let rhs = rhs.borrow();
                            lhs.push_str(rhs.as_str());
                            self.stack.push(lhs.as_value());
                        }
                        _ => unimplemented!(),
                    }
                }
                OpCode::Sub => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();

                    match lhs {
                        Value::Number(lhs) => {
                            let Value::Number(rhs) = rhs else {
                                panic!()
                            };
                            self.stack.push(Value::Number(lhs - rhs))
                        }
                        x => unimplemented!("cannot subtract value {}", x),
                    }
                }
                OpCode::Mul => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();

                    match lhs {
                        Value::Number(lhs) => {
                            let Value::Number(rhs) = rhs else {
                                panic!()
                            };
                            self.stack.push(Value::Number(lhs * rhs))
                        }
                        _ => unimplemented!(),
                    }
                }
                OpCode::Pop => unsafe {
                    self.stack.set_len(self.stack.len() - 1);
                },
                OpCode::Div => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();

                    match lhs {
                        Value::Number(lhs) => {
                            let Value::Number(rhs) = rhs else {
                                panic!()
                            };
                            self.stack.push(Value::Number(lhs / rhs))
                        }
                        _ => unimplemented!(),
                    }
                }
                OpCode::Print => {
                    println!("{}", self.stack.pop().unwrap());
                }
                OpCode::AssertEq => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();

                    assert_eq!(lhs, rhs);
                }
                OpCode::AssertNe => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();

                    assert_ne!(lhs, rhs);
                }
                OpCode::Return => {
                    let mut returning = self.stack.pop();
                    self.frame_count -= 1;

                    if self.frame_count == 0 {
                        self.stack.pop();
                        println!("{}", start.elapsed().as_secs_f64());
                        return;
                    }

                    current_frame = &self.callframes[self.frame_count - 1];
                    function = &current_frame.function;
                    chunk = &function.chunk;
                    ip = current_frame.ip;
                    self.stack.truncate(self.callframes[self.frame_count].slots);
                    self.stack.push(returning.take().unwrap());
                }
                OpCode::Call(arg_count) => {
                    let callee = &self.stack[self.stack.len() - (1 + arg_count)];
                    self.call(
                        match callee {
                            Value::Function(ptr) => {
                                let ptr = ptr.as_ref();
                                let function = &*ptr.borrow();

                                function.clone()
                            }
                            x => panic!("got {:?}", x),
                        },
                        arg_count,
                    );
                    self.callframes[self.frame_count - 2].ip = ip;

                    // prepares for the next callframe
                    {
                        current_frame = &self.callframes[self.frame_count - 1];
                        function = &current_frame.function;
                        chunk = &function.chunk;
                        ip = 0;
                    }
                }

                OpCode::Less => {
                    let Some(Value::Number(rhs)) = self.stack.pop() else {
                        panic!()
                    };
                    let tmp = self.stack.pop();
                    let Some(Value::Number(lhs)) = tmp else {
                        panic!("{:?}", tmp)
                    };
                    self.stack.push((lhs < rhs).as_value())
                }
                OpCode::LessEq => {
                    let Some(Value::Number(rhs)) = self.stack.pop() else {
                        panic!()
                    };
                    let Some(Value::Number(lhs)) = self.stack.pop() else {
                        panic!()
                    };
                    self.stack.push((lhs <= rhs).as_value())
                }
                OpCode::Greater => {
                    let Some(Value::Number(rhs)) = self.stack.pop() else {
                        panic!()
                    };
                    let Some(Value::Number(lhs)) = self.stack.pop() else {
                        panic!()
                    };
                    self.stack.push((lhs > rhs).as_value())
                }
                OpCode::GreaterEq => {
                    let Some(Value::Number(rhs)) = self.stack.pop() else {
                        panic!()
                    };
                    let Some(Value::Number(lhs)) = self.stack.pop() else {
                        panic!()
                    };
                    self.stack.push((lhs >= rhs).as_value())
                }
            }
        }
    }
}
