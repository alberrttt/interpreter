use crate::{
    common::{
        opcode::OpCode,
        value::{rcrf, AsValue, Value},
    },
    frontend::compiler::Compiler,
};

use super::{
    expression::{AsExpr, Expression},
    node::Node,
    CompileToBytecode,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
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
        match self {
            Literal::Number(number) => number,
            _ => panic!(),
        }
    }
}
impl From<Literal> for Value {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::Void => Value::Void,
            Literal::Number(num) => Value::Number(num),
            Literal::String(string) => Value::String(rcrf(string)),
            Literal::Bool(bool) => Value::Boolean(bool),
        }
    }
}

impl CompileToBytecode for Literal {
    fn to_bytecode(self, compiler: &mut Compiler) {
        let function = &mut compiler.function;
        let pos = match self {
            Literal::Void => function.chunk.emit_value(Value::Void),
            Literal::Number(number) => function.chunk.emit_value(Value::Number(number)),
            Literal::String(string) => function.chunk.emit_value(string.to_value()),
            Literal::Bool(bool) => {
                if bool {
                    function.chunk.emit_op(OpCode::True)
                } else {
                    function.chunk.emit_op(OpCode::False)
                };
                return;
            }
        };
        function.chunk.emit_op(OpCode::Constant(pos))
    }
}
impl AsExpr for Literal {
    fn to_expr(self) -> super::expression::Expression {
        Expression::Literal(self)
    }
}
