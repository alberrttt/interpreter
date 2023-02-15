use std::{collections::HashMap, rc::Rc, time::Instant};

use colored::Colorize;

use crate::{
    backend::vm::natives::NATIVES,
    common::{
        chunk::Chunk,
        closure::{self, Closure},
        debug::diassasemble_instruction,
        function::Function,
        interner::InternedString,
        natives::Native,
        opcode::OpCode,
        value::{AsValue, RuntimeUpvalue, Value},
    },
};

use self::natives::NATIVES_LEN;

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
    upvalue_count: 0,
};

#[derive(Debug)]
pub struct VirtualMachine {
    pub stack: Vec<Value>,
    pub callframes: [CallFrame; 2048],
    pub frame_count: usize,
    pub globals: HashMap<usize, Value>,
    pub natives: &'static [Native; NATIVES_LEN],
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        pub const CALLFRAME: CallFrame = CallFrame {
            closure: std::ptr::null(),
            ip: 0,
            slots: 0,
        };
        VirtualMachine {
            callframes: [CALLFRAME; 2048],
            stack: vec![],
            natives: &NATIVES,
            globals: HashMap::new(),
            frame_count: 0,
        }
    }
    #[allow(clippy::not_unsafe_ptr_arg_deref, unsafe_code)]
    pub fn call(&mut self, closure: *const Closure, arg_count: usize) {
        let function = unsafe { &(*closure).func };
        let arity = function.arity;
        if arg_count != arity as usize {
            panic!("mismatched argument counts! expected {arity} got {arg_count}");
        }

        let frame: &mut CallFrame = &mut self.callframes[self.frame_count];

        frame.closure = closure;
        frame.slots = self.stack.len() - (arg_count + 1);

        self.frame_count += 1;
    }
    pub fn run(mut self) {
        let start = Instant::now();
        let mut current_frame = &self.callframes[self.frame_count - 1] as *const CallFrame;
        macro_rules! current_frame {
            () => {{
                #[allow(unsafe_code)]
                unsafe {
                    &*current_frame
                }
            }};
        }
        macro_rules! read_current_closure {
            () => {{
                unsafe { &(*current_frame!().closure) }
            }};
        }
        macro_rules! read_current_frame_fn {
            () => {{
                #[allow(unsafe_code)]
                unsafe {
                    &*read_current_closure!().func
                }
            }};
        }
        let mut closure = read_current_closure!();
        let mut function = read_current_frame_fn!();
        let mut chunk = &function.chunk;
        let mut ip: usize = 0;
        macro_rules! pop {
            () => {{
                assert!(self.stack.len() > 0);

                let i = self.stack.len() - 1;
                let tmp = ::std::mem::take(&mut self.stack[i]);
                #[allow(unsafe_code)]
                unsafe {
                    self.stack.set_len(i);
                }

                tmp
            }};
        }
        macro_rules! binary_op {
            ($op:tt) => {{
                let rhs = pop!();
                let tmp = self.stack.len() - 1;
                let lhs = &mut self.stack[tmp];

                match lhs {
                    Value::Number(lhs) => {
                        let Value::Number(rhs) = rhs else {
                            panic!()
                        };
                        *lhs = *lhs $op rhs;
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

            ip += 1;

            match instruction.clone() {
                OpCode::Byte(_) => {}
                OpCode::SetUpValue(u) => {}
                OpCode::GetUpValue(u) => {
                    let tmp = closure.upvalues[u as usize].location.as_ref().clone();

                    self.stack.push(tmp);
                }
                OpCode::Closure(location) => {
                    let function = &chunk.constants[location as usize];

                    let Value::Function(function) = function else {
                        panic!("{:?}", function)
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
                            let value =
                                self.stack[index as usize + 1 + current_frame!().slots].clone();
                            closure.upvalues.push(RuntimeUpvalue {
                                location: Box::new(value),
                            })
                        }
                    }

                    self.stack.push(Value::Closure(Box::new(closure)))
                }
                OpCode::SetLocalConsumes(index) => {
                    self.stack[index as usize + 1 + current_frame!().slots] = pop!();
                }
                OpCode::Equal => {
                    binary_op_bool!(==)
                }
                OpCode::NotEqual => {
                    binary_op_bool!(!=)
                }
                OpCode::CallFnArgPtr(location, args) => {
                    // generate code only for CallFnArgPtr
                    let native = &self.natives[location as usize];
                    // rewrite the following line, but pop the args from the stack
                    let drain: Vec<_> = self
                        .stack
                        .drain(self.stack.len() - args as usize..)
                        .collect();
                    let mut args = drain.to_vec();
                    {
                        (native.0)(&mut self, args);
                    }
                }
                OpCode::CallNative(location) => {
                    let native = &self.natives[location as usize];
                    let mut args: Vec<Value> = vec![];
                    {
                        (native.0)(&mut self, args);
                    }
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
                    let value = self.stack[index as usize + 1 + current_frame!().slots].clone();
                    self.stack.push(value)
                }
                OpCode::SetLocal(index) => {
                    self.stack[index as usize + 1 + current_frame!().slots] =
                        self.stack.last().unwrap().clone();
                }
                OpCode::DefineLocal(location) => {
                    self.stack.push(chunk.constants[location as usize].clone())
                }
                OpCode::GetGlobal(name) => {
                    let name = chunk.constants[name as usize].as_string();
                    let value = self.globals.get(&name.0).unwrap().clone();
                    self.stack.push(value)
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
                    let tmp = self.stack.len() - 1;
                    let lhs = &mut self.stack[tmp];

                    match lhs {
                        Value::Number(lhs) => {
                            let Value::Number(rhs) = rhs else {
                                panic!()
                            };
                            *lhs += rhs;
                        }
                        Value::String(string_ref) => {
                            let Value::String(rhs) = rhs else {
                                panic!("lhs {:?}\nrhs{:?}",lhs,rhs);
                            };

                            let mut lhs: String = (*string_ref).into();
                            let rhs: String = rhs.into();
                            lhs.push_str(rhs.as_str());
                            *string_ref = InternedString::from(lhs.as_ref());
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
                    drop(std::mem::take({
                        let tmp = self.stack.len() - 1;
                        &mut self.stack[tmp]
                    }));
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

                    closure = read_current_closure!();

                    function = read_current_frame_fn!();
                    chunk = &function.chunk;
                    ip = current_frame!().ip;
                    self.stack.truncate(self.callframes[self.frame_count].slots);
                    self.stack.push(returning);
                }
                // room for improvement
                OpCode::Call(arg_count) => {
                    let tmp = self.stack.len() - (1 + arg_count);
                    let callee = std::mem::take(&mut self.stack[tmp]);

                    let Value::Closure(callee) = callee else {
                            panic!()
                        };

                    self.call(Box::into_raw(callee), arg_count);
                    self.callframes[self.frame_count - 2].ip = ip;

                    // prepares for the next callframe
                    {
                        current_frame = &self.callframes[self.frame_count - 1];
                        closure = read_current_closure!();
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

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}
