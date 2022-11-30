use crate::common::function::Function;

use super::{
    ast::{node::Node, CompileToBytecode},
    compiler::Compiler,
};
#[derive(Default, Debug)]
pub struct FileNode<'a> {
    pub nodes: Vec<Node>,
    pub compiler: Option<Compiler<'a>>,
}

impl<'a> FileNode<'a> {
    pub fn build_function(mut self) -> Function {
        let mut compiler = self.compiler.take().unwrap();
        let function = Function::new();
        compiler.function = function;
        for node in self.nodes {
            node.to_bytecode(&mut compiler)
        }
        let function = compiler.function;
        compiler.function = Function::new();
        self.compiler = Some(compiler);
        function
    }
}
