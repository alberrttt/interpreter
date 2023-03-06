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

use super::{AsExpr, Expression};

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub declarations: Vec<Node>,
}
impl From<Block> for Expression {
    fn from(value: Block) -> Self {
        Expression::Block(value)
    }
}
impl From<Block> for Node {
    fn from(value: Block) -> Self {
        Node::Expression(value.into())
    }
}
impl CompileToBytecode for Block {
    fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        compiler.begin_scope();
        for dec in &self.declarations {
            dec.to_bytecode(compiler)
        }
        compiler.end_scope();
    }
}
impl<'a> Compiler<'a> {
    pub fn begin_scope(&mut self) {
        self.bytecode.scope_depth += 1;
    }
    pub fn end_scope(&mut self) {
        self.bytecode.scope_depth -= 1;
        while self.bytecode.local_count > 0
            && self.bytecode.locals[self.bytecode.local_count - 1].depth > self.bytecode.scope_depth
        {
            if self.bytecode.locals[self.bytecode.local_count - 1].is_captured {
                self.bytecode.write_close_upvalue_op()
            } else {
                self.bytecode.function.chunk.emit_op(OpCode::Pop);
            }
            self.bytecode.local_count -= 1;
        }

        self.bytecode
            .function
            .chunk
            .emit_many(std::mem::take(&mut self.bytecode.emit_after_block))
    }
}
impl AsExpr for Block {
    fn to_expr(self) -> super::Expression {
        super::Expression::Block(self)
    }
}
impl AsNode for Block {
    fn to_node(self) -> crate::frontend::ast::node::Node {
        self.to_expr().to_node()
    }
}
