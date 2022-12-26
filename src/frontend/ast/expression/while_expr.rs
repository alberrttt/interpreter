use crate::{common::opcode::OpCode, frontend::ast::CompileToBytecode};

use super::{block::Block, AsExpr, Expression};

#[derive(Debug, Clone, PartialEq)]
pub struct WhileExpr {
    pub predicate: Box<Expression>,
    pub block: Block,
}
impl AsExpr for WhileExpr {
    fn as_expr(self) -> Expression {
        Expression::While(self)
    }
}
/// GUAGE YOUR EYES OUT
impl CompileToBytecode for WhileExpr {
    fn to_bytecode(self, compiler: &mut crate::frontend::compiler::Compiler) -> () {
        let predicate = compiler.function.chunk.code.len();

        self.predicate.to_bytecode(compiler);
        let predicate_jump = compiler.emit_pop_jump_if_false();

        self.block.to_bytecode(compiler);
        let loop_jump = {
            compiler.function.chunk.emit_op(OpCode::JumpTo(predicate));
            compiler.function.chunk.code.len() - 1
        };
        compiler.function.chunk.code[predicate_jump] = OpCode::PopJumpToIfFalse(loop_jump + 1);
    }
}
