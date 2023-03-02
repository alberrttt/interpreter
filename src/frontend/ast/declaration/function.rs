use std::{rc::Rc};

use crate::{
    common::{
        opcode::OpCode,
        value::{AsValue, Value},
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
        let lexeme = self.name.value.lexeme.clone();
        let _name = compiler
            .bytecode
            .function
            .chunk
            .emit_value(lexeme.to_value());
        compiler.bytecode.globals.push(lexeme);
        let mut temp_compiler = Compiler::new(compiler.diagnostics.clone(), FunctionType::Function);
        temp_compiler.enclosing = Some(Enclosing(compiler));
        temp_compiler
            .bytecode
            .globals
            .extend(compiler.bytecode.globals.clone());
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
            if !temp_compiler.bytecode.returned {
                temp_compiler.bytecode.write_void_op();
                temp_compiler
                    .bytecode
                    .function
                    .chunk
                    .emit_op(OpCode::Return);
            }
            temp_compiler.bytecode.function
        };
        let function = Rc::new(function);
        let location = compiler
            .bytecode
            .function
            .chunk
            .emit_value(Value::Function(function.clone()));
        compiler.bytecode.write_closure_op(location);

        let count = function.upvalue_count;
        let upvalues = &temp_compiler.bytecode.upvalues[..count].to_vec();
        for upvalue in upvalues {
            compiler.bytecode.write_byte(upvalue.is_local as u8);
            compiler.bytecode.write_byte(upvalue.index)
        }
        if compiler.in_scope() {
            compiler.add_local(self.name.value.clone());
        } else {
            // location of the name in the constant pool
            let lexeme = self.name.value.lexeme.clone();
            let name = compiler
                .bytecode
                .function
                .chunk
                .emit_value(lexeme.to_value());
            compiler.bytecode.globals.push(lexeme);
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
