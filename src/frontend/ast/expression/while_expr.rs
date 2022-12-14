

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
        let predicate_location = compiler.function.chunk.code.len();
        self.predicate.to_bytecode(compiler);
        let jump_op = compiler.function.chunk.code.len();
        compiler
            .function
            .chunk
            .emit_op(OpCode::JumpToIfFalse(0xfff));
        compiler.function.chunk.emit_op(OpCode::Pop);
        self.block.to_bytecode(compiler);
        let _after_block = compiler.function.chunk.code.len();
        compiler
            .function
            .chunk
            .emit_op(OpCode::JumpTo(predicate_location));
        let pop_location = compiler.function.chunk.code.len();
        compiler.function.chunk.emit_op(OpCode::Pop);
        compiler.function.chunk.code[jump_op] = OpCode::JumpToIfFalse(pop_location);
    }
}
