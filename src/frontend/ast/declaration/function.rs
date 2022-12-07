use crate::{
    common::{
        function,
        opcode::OpCode,
        value::{rcrf, AsValue, Value},
    },
    frontend::{
        ast::{expression::block::Block, identifier::Identifier, CompileToBytecode},
        compiler::{Compiler, FunctionType},
    },
};

use super::AsDeclaration;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub block: Block,
}

impl CompileToBytecode for FunctionDeclaration {
    fn to_bytecode(self, compiler: &mut crate::frontend::compiler::Compiler) -> () {
        let context = compiler.context.take().unwrap();
        let mut temp_compiler = Compiler::new(context, FunctionType::Function);
        self.block.to_bytecode(&mut temp_compiler);
        let function = temp_compiler.function;
        compiler
            .function
            .chunk
            .emit_constant(Value::Function(rcrf(function)));
        if compiler.scope_depth > 0 {
            compiler.add_local(self.name.value);
            return;
        } else {
            let name = compiler
                .function
                .chunk
                .emit_value(self.name.value.lexeme.as_value().clone());
            compiler.function.chunk.emit_op(OpCode::DefineGlobal(name))
        };
        compiler.context = temp_compiler.context.take();
    }
}
impl AsDeclaration for FunctionDeclaration {
    fn as_declaration(self) -> super::Declaration {
        super::Declaration::FunctionDeclaration(self)
    }
}
