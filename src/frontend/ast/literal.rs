use crate::{
    common::{
        interner::STRING_INTERNER,
        opcode::OpCode,
        value::{AsValue, Value},
    },
    frontend::{compiler::Compiler, scanner::Token},
};

use super::{
    expression::{AsExpr, Expression},
    node::Node,
    CompileToBytecode,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Literal(pub Literals, pub Token);
#[derive(Debug, PartialEq, Clone)]
pub enum Literals {
    Number(f64),
    String(String),
    Bool(bool),
    Void,
}
impl Literal {
    pub fn as_node(self) -> Node {
        Node::Literal(self)
    }
    pub fn as_number(self) -> f64 {
        match self.0 {
            Literals::Number(number) => number,
            _ => panic!(),
        }
    }
}
impl From<Literal> for Value {
    fn from(literal: Literal) -> Self {
        match literal.0 {
            Literals::Void => Value::Void,
            Literals::Number(num) => Value::Number(num),
            Literals::String(string) => {
                let mut interner = STRING_INTERNER.lock().expect("already?");
                Value::String(interner.get_or_intern(&string))
            }
            Literals::Bool(bool) => Value::Boolean(bool),
        }
    }
}

impl CompileToBytecode for Literal {
    fn to_bytecode(&self, compiler: &mut Compiler) {
        let function = &mut compiler.bytecode.function;
        let pos = match &self.0 {
            Literals::Void => function.chunk.emit_value(Value::Void),
            Literals::Number(number) => function.chunk.emit_value(Value::Number(*number)),
            Literals::String(string) => function.chunk.emit_value(string.to_value()),
            Literals::Bool(bool) => {
                if *bool {
                    function.chunk.emit_op(OpCode::True)
                } else {
                    function.chunk.emit_op(OpCode::False)
                };
                return;
            }
        };
        compiler.bytecode.write_constant_op(pos);
    }
}
impl AsExpr for Literal {
    fn to_expr(self) -> super::expression::Expression {
        Expression::Literal(self)
    }
}
