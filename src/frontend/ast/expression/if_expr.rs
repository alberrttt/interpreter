use crate::{
    common::opcode::OpCode,
    frontend::{ast::CompileToBytecode, compiler::Compiler},
};

use super::{block::Block, AsExpr, Expression};

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub predicate: Box<Expression>,
    pub then: Block,
    pub else_block: Option<Block>,
}
impl AsExpr for If {
    fn to_expr(self) -> super::Expression {
        super::Expression::If(self)
    }
}

/// GUAGE YOUR EYES OUT
impl CompileToBytecode for If {
    fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        self.predicate.to_bytecode(compiler);
        let predicate_jump = compiler.emit_pop_jump_if_false();

        self.then.to_bytecode(compiler);

        // where the then block ends
        let then_end = compiler.bytecode.function.chunk.code.len();
        compiler
            .bytecode
            .function
            .chunk
            .emit_op(OpCode::JumpTo(then_end + 1));

        if let Some(else_block) = &self.else_block {
            else_block.to_bytecode(compiler);
            let after_else = compiler.bytecode.function.chunk.code.len();
            compiler.bytecode.function.chunk.code[then_end] = OpCode::JumpTo(after_else);
        }

        compiler.bytecode.function.chunk.code[predicate_jump] =
            OpCode::PopJumpToIfFalse(then_end + 1);
    }
}

impl<'a> Compiler<'a> {
    pub fn emit_pop_jump_if_false(&mut self) -> usize {
        let jump_op = self.bytecode.function.chunk.code.len();
        self.bytecode
            .function
            .chunk
            .emit_op(OpCode::PopJumpToIfFalse(0xfff));
        jump_op
    }
}
