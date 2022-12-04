use std::ops::Sub;

use crate::{common::opcode::OpCode, frontend::ast::CompileToBytecode};

use super::{block::Block, AsExpr, Expression};

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub condition: Box<Expression>,
    pub then: Block,
    pub else_block: Option<Block>,
}
impl AsExpr for IfExpr {
    fn as_expr(self) -> super::Expression {
        super::Expression::If(self)
    }
}
impl CompileToBytecode for IfExpr {
    fn to_bytecode(self, compiler: &mut crate::frontend::compiler::Compiler) -> () {
        self.condition.to_bytecode(compiler);
        let jump_op = compiler.function.chunk.code.len();
        compiler.function.chunk.emit_op(OpCode::JumpIfFalse(0xfff));
        compiler.function.chunk.emit_op(OpCode::Pop);
        self.then.to_bytecode(compiler);
        let skip = compiler.function.chunk.code.len();
        compiler.function.chunk.emit_op(OpCode::Jump(1));
        compiler.function.chunk.emit_op(OpCode::Pop);
        let after = compiler.function.chunk.code.len();
        if let Some(else_block) = self.else_block {
            else_block.to_bytecode(compiler);
            let after_else = compiler.function.chunk.code.len();
            let to_skip = after_else.sub(after);
            compiler.function.chunk.code[skip] = OpCode::Jump((to_skip) as u16);
        }

        let offset = after.sub(jump_op);
        compiler.function.chunk.code[jump_op] = OpCode::JumpIfFalse((offset - 2) as u16);
    }
}
