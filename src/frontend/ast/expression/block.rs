use crate::{
    common::opcode::OpCode,
    frontend::{
        ast::{
            node::{AsNode, Node},
            CompileToBytecode,
        },
        compiler::Compiler,
    },
};

use super::AsExpr;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub declarations: Vec<Node>,
}

impl CompileToBytecode for Block {
    fn to_bytecode(self, compiler: &mut crate::frontend::compiler::Compiler) -> () {
        compiler.begin_scope();
        for dec in self.declarations {
            dec.to_bytecode(compiler)
        }
        compiler.end_scope();
    }
}
impl<'a> Compiler<'a> {
    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }
    pub fn end_scope(&mut self) {
        self.scope_depth -= 1;
        while self.local_count > 0 && self.locals[self.local_count - 1].depth > self.scope_depth {
            self.function.chunk.emit_op(OpCode::Pop);
            self.local_count -= 1;
        }
    }
}
impl AsExpr for Block {
    fn as_expr(self) -> super::Expression {
        super::Expression::Block(self)
    }
}
impl AsNode for Block {
    fn as_node(self) -> crate::frontend::ast::node::Node {
        self.as_expr().as_node()
    }
}
