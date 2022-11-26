use crate::common::function::Function;

use super::ast::{node::Node, CompileToBytecode};
#[derive(Default, Debug)]
pub struct FileNode {
    pub nodes: Vec<Node>,
}

impl FileNode {
    pub fn build_function(self) -> Function {
        let mut function = Function::new();
        for node in self.nodes {
            node.to_bytecode(&mut function)
        }
        function
    }
}
