use std::collections::HashMap;

use crate::common::{
    chunk::Chunk,
    opcode::OpCode,
    value::{AsValue, Value},
};

pub mod ops;

pub struct VM {
    pub stack: Vec<Value>,
    pub globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: vec![],
            globals: HashMap::new(),
        }
    }
    pub fn run(&mut self, chunk: Chunk) {
        let mut ip: usize = 0;
        loop {
            let instruction = &chunk.code[ip];
            ip += 1;
            match instruction {
                OpCode::True => self.stack.push(true.as_value()),
                OpCode::False => self.stack.push(false.as_value()),
                OpCode::Constant(index) => {
                    self.stack.push(chunk.constants[*index as usize].clone())
                }
                OpCode::GetGlobal(name) => {
                    let name = chunk.constants[*name as usize].as_string();
                    self.stack.push(self.globals.get(name).unwrap().clone())
                }
                OpCode::SetGlobal(name) => {
                    let name = (chunk.constants[*name as usize].as_string()).to_owned();
                    assert!(self.globals.contains_key(&name));
                    let value = self.stack.pop().unwrap();
                    self.globals.insert(name, value);
                }
                OpCode::DefineGlobal(name) => {
                    let name = (chunk.constants[*name as usize].as_string()).to_owned();
                    let value = self.stack.pop().unwrap();
                    self.globals.insert(name, value);
                }
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
                                panic!()
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
                        _ => unimplemented!(),
                    }
                }
                OpCode::Mul => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();

                    match lhs {
                        Value::Number(lhs) => {
                            let Value::Number(_rhs) = rhs else {
                                panic!()
                            };
                            self.stack.push(Value::Number(lhs * 1.0))
                        }
                        _ => unimplemented!(),
                    }
                }
                OpCode::Pop => {
                    self.stack.pop();
                }
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
                OpCode::Return => {
                    break;
                }
            }
        }
    }
}
