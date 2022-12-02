use crate::{
    common::{opcode::OpCode, value::AsValue},
    frontend::{compiler::Compiler, scanner::Token},
};

use super::{
    node::{AsNode, Node},
    CompileToBytecode,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: Token,
}

impl CompileToBytecode for Identifier {
    fn to_bytecode(self, compiler: &mut Compiler) -> () {
        let Compiler {
            function,
            scanner: _,
            parser: _,
            context: _,
            scope_depth,
            locals,
            enclosing,
        } = compiler;
        let name = function.chunk.emit_value(self.name.value.as_value());
        function.chunk.emit_op(OpCode::GetGlobal(name));
    }
}

impl AsNode for Identifier {
    fn as_node(self) -> super::node::Node {
        Node::Identifier(self)
    }
}
