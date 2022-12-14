

use crate::{common::opcode::OpCode, frontend::ast::CompileToBytecode};

use super::{block::Block, AsExpr, Expression};

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub predicate: Box<Expression>,
    pub then: Block,
    pub else_block: Option<Block>,
}
impl AsExpr for IfExpr {
    fn as_expr(self) -> super::Expression {
        super::Expression::If(self)
    }
}

/// GUAGE YOUR EYES OUT
impl CompileToBytecode for IfExpr {
    fn to_bytecode(self, compiler: &mut crate::frontend::compiler::Compiler) -> () {
        self.predicate.to_bytecode(compiler);
        let jump_op = compiler.function.chunk.code.len();
        compiler
            .function
            .chunk
            .emit_op(OpCode::JumpToIfFalse(0xfff));
        compiler.function.chunk.emit_op(OpCode::Pop);
        self.then.to_bytecode(compiler);
        let skip = compiler.function.chunk.code.len();
        compiler.function.chunk.emit_op(OpCode::JumpTo(skip + 2));
        let pop = compiler.function.chunk.code.len();
        compiler.function.chunk.emit_op(OpCode::Pop);
        if let Some(else_block) = self.else_block {
            else_block.to_bytecode(compiler);
            let after_else = compiler.function.chunk.code.len();
            compiler.function.chunk.code[skip] = OpCode::JumpTo(after_else);
        }

        compiler.function.chunk.code[jump_op] = OpCode::JumpToIfFalse(pop);
    }
}
