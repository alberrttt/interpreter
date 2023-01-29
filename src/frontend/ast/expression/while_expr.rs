use crate::{common::opcode::OpCode, frontend::ast::CompileToBytecode};

use super::{block::Block, AsExpr, Expression};

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub predicate: Box<Expression>,
    pub block: Block,
}
impl AsExpr for While {
    fn to_expr(self) -> Expression {
        Expression::While(self)
    }
}
/// GUAGE YOUR EYES OUT
impl CompileToBytecode for While {
    fn to_bytecode(&self, compiler: &mut crate::frontend::compiler::Compiler) {
        let predicate = compiler.bytecode.function.chunk.code.len();

        self.predicate.to_bytecode(compiler);
        let predicate_jump = compiler.emit_pop_jump_if_false();

        self.block.to_bytecode(compiler);
        let loop_jump = {
            compiler
                .bytecode
                .function
                .chunk
                .emit_op(OpCode::JumpTo(predicate));
            compiler.bytecode.function.chunk.code.len() - 1
        };
        compiler.bytecode.function.chunk.code[predicate_jump] =
            OpCode::PopJumpToIfFalse(loop_jump + 1);
    }
}
