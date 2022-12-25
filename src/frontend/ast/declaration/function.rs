use crate::{
    common::{
        debug::dissasemble_chunk,
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
    pub parameters: Vec<Identifier>,
}
impl<'a> Compiler<'a> {
    fn in_scope(&self) -> bool {
        self.scope_depth > 0
    }
}
impl CompileToBytecode for FunctionDeclaration {
    fn to_bytecode(self, compiler: &mut crate::frontend::compiler::Compiler) -> () {
        // uses the current compiler's compilation context for the function
        // which is returned later
        let mut temp_compiler = Compiler::new(
            compiler.interner.clone(),
            compiler.context.take().unwrap(),
            FunctionType::Function,
        );
        let function = {
            // sets the function name and arity
            temp_compiler.function.arity = self.parameters.len() as u8;
            temp_compiler.function.name = self.name.value.lexeme.clone();

            // tells the compiler to recongize any parameters
            for param in self.parameters {
                temp_compiler.add_local(param.value)
            }

            // finally compiles the block
            self.block.to_bytecode(&mut temp_compiler);

            temp_compiler.function.chunk.emit_op(OpCode::Return);
            temp_compiler.function
        };
        compiler.context = temp_compiler.context.take();
        if compiler.context.as_ref().unwrap().flags.display_bytecode {
            dissasemble_chunk(&function.chunk);
        }
        compiler
            .function
            .chunk
            .emit_constant(Value::Function(rcrf(function)));

        if compiler.in_scope() {
            compiler.add_local(self.name.value);
            return;
        } else {
            // location of the name in the constant pool
            let name = compiler
                .function
                .chunk
                .emit_value(self.name.value.lexeme.as_value().clone());
            compiler.function.chunk.emit_op(OpCode::DefineGlobal(name))
        };
        // compilation context is returned
    }
}
impl AsDeclaration for FunctionDeclaration {
    fn as_declaration(self) -> super::Declaration {
        super::Declaration::FunctionDeclaration(self)
    }
}
