use crate::common::value::Value;

use super::{
    ast::{node::Node, CompileToBytecode},
    compiler::Compiler,
};
#[derive(Default, Debug)]
pub struct FileNode<'a> {
    pub nodes: Vec<Node>,
    pub compiler: Option<Compiler<'a>>,
    pub file_attributes: FileAttributes,
}

#[derive(Debug, Default)]
pub struct FileAttributes {
    pub expect_stack: Option<Vec<Value>>,
}

impl<'a> CompileToBytecode for FileNode<'a> {
    fn to_bytecode(self, compiler: &mut Compiler) {
        for node in self.nodes {
            node.to_bytecode(compiler)
        }
    }
}
