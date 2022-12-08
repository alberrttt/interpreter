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
        let mut misc_slots: [Value; 8] = [
            Value::None,
            Value::None,
            Value::None,
            Value::None,
            Value::None,
            Value::None,
            Value::None,
            Value::None,
        ];
        let mut jump_exhaust: usize = 0;
        loop {
            let instruction = &chunk.code[ip as usize];
            ip += 1;
            #[cfg(debug_assertions)]
            {
                if match instruction {
                    OpCode::JumpTo(_) | OpCode::JumpToIfFalse(_) => true,
                    _ => false,
                } {
                    if jump_exhaust > 20 {
                        panic!("Jump exhuasted!");
                    }
                } else if jump_exhaust > 0 {
                    jump_exhaust -= 1;
                }
                println!("{ip} Executing {instruction} ")
            }
            match instruction {
                OpCode::JumpTo(offset) => {
                    ip = *offset as usize;
                    jump_exhaust += 1;
                }
                OpCode::JumpToIfFalse(offset) => {
                    let condition = self.stack.last().unwrap().as_bool();
                    if !condition {
                        ip = *offset as usize;
                    }
                    jump_exhaust += 1;
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
                OpCode::True => self.stack.push(true.as_value()),
                OpCode::False => self.stack.push(false.as_value()),
                OpCode::Constant(location) => {
                    self.stack.push(chunk.constants[*location as usize].clone())
                }
                OpCode::GetLocal(index) => self.stack.push(self.stack[*index as usize].clone()),
                OpCode::SetLocal(index) => {
                    self.stack[*index as usize] = self.stack.last().unwrap().clone();
                }
                OpCode::DefineLocal(location) => {
                    self.stack.push(chunk.constants[*location as usize].clone())
                }
                OpCode::GetGlobal(name) => {
                    let name = chunk.constants[*name as usize].as_string();
                    self.stack.push(self.globals.get(name).unwrap().clone())
                }
                OpCode::SetGlobal(name) => {
                    let name = (chunk.constants[*name as usize].as_string()).to_owned();
                    assert!(self.globals.contains_key(&name));
                    let value = self.stack[self.stack.len() - 1].clone();
                    self.globals.insert(name, value);
                }
                OpCode::DefineGlobal(name) => {
                    let name = (chunk.constants[*name as usize].as_string()).to_owned();
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
                            let Value::Number(rhs) = rhs else {
                                panic!()
                            };
                            self.stack.push(Value::Number(lhs * rhs))
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
                    break;
                }
                OpCode::TakeTempSlot(slot) => {
                    let slot = std::mem::replace(&mut misc_slots[*slot as usize], Value::None);
                    self.stack.push(slot)
                }
                OpCode::SetTempSlot(slot) => {
                    misc_slots[*slot as usize] = self.stack.pop().unwrap();
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
