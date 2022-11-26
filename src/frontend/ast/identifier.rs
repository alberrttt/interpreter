use crate::common::{opcode::OpCode, value::AsValue};

use super::{
    node::{AsNode, Node},
    CompileToBytecode,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
}

impl CompileToBytecode for Identifier {
    fn to_bytecode(self, function: &mut crate::common::function::Function) -> () {
        let name = function.chunk.emit_value(self.name.as_value());
        function.chunk.emit_op(OpCode::GetGlobal(name));
    }
}

impl AsNode for Identifier {
    fn as_node(self) -> super::node::Node {
        Node::Identifier(self)
    }
}
