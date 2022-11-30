use crate::frontend::{
    ast::{declaration::Declaration, node::AsNode, CompileToBytecode},
    compiler::Compiler,
};

use super::AsExpr;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub declarations: Vec<Declaration>,
}

impl CompileToBytecode for Block {
    fn to_bytecode(self, compiler: &mut crate::frontend::compiler::Compiler) -> () {
        todo!()
    }
}
impl<'a> Compiler<'a> {
    pub fn begin_scope(&mut self) {}
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
