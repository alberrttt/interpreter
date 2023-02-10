use std::{borrow::Borrow, rc::Rc};

use crate::{
    common::{
        opcode::OpCode,
        value::{rcrf, AsValue, Value},
    },
    frontend::{
        ast::{expression::block::Block, identifier::Identifier, CompileToBytecode},
        compiler::{Compiler, Enclosing, FunctionType},
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
        self.bytecode.scope_depth > 0
    }
}
impl CompileToBytecode for FunctionDeclaration {
    fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        // uses the current compiler's compilation context for the function
        // which is returned later
        let mut temp_compiler = Compiler::new(compiler.diagnostics.clone(), FunctionType::Function);
        temp_compiler.enclosing = Some(Enclosing(compiler));
        let function = {
            // sets the function name and arity
            temp_compiler.bytecode.function.arity = self.parameters.len() as u8;
            temp_compiler.bytecode.function.name = self.name.value.lexeme.clone();

            // tells the compiler to recongize any parameters
            for param in &self.parameters {
                temp_compiler.add_local(param.value.clone())
            }

            // finally compiles the block
            self.block.to_bytecode(&mut temp_compiler);
            // unecessary return if the source code for the function already includes one
            // i.e `func x() {return 1;}` will have two return ops
            temp_compiler
                .bytecode
                .function
                .chunk
                .emit_op(OpCode::Return);
            temp_compiler.bytecode.function
        };
        let function = Rc::new(function);
        let location = compiler
            .bytecode
            .function
            .chunk
            .emit_value(Value::Function(function.clone()));
        compiler.bytecode.write_closure_op(location);
        dbg!(&compiler.bytecode.upvalues);
        dbg!(temp_compiler.bytecode.upvalues);
        for upvalue in compiler.bytecode.upvalues.clone().iter() {
            compiler.bytecode.write_byte(upvalue.is_local as u8);
            compiler.bytecode.write_byte(upvalue.index)
        }
        if compiler.in_scope() {
            compiler.add_local(self.name.value.clone());
        } else {
            // location of the name in the constant pool
            let name = compiler
                .bytecode
                .function
                .chunk
                .emit_value(self.name.value.lexeme.to_value());
            compiler
                .bytecode
                .function
                .chunk
                .emit_op(OpCode::DefineGlobal(name))
        };
        // compilation context is returned
    }
}
impl AsDeclaration for FunctionDeclaration {
    fn to_declaration(self) -> super::Declaration {
        super::Declaration::FunctionDeclaration(self)
    }
}
