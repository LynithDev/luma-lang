use luma_diagnostic::CompilerResult;

use crate::{ast::*, bytecode::*, stages::codegen::ctx::BytecodeGenCtx};

pub struct BytecodeGen;

impl BytecodeGen {
    pub fn generate(ast: Ast) -> CompilerResult<Bytecode> {
        let mut ctx = BytecodeGenCtx::default();
        let mut ast = ast;

        Self.traverse(&mut ctx, &mut ast);

        Ok(Bytecode {
            instructions: ctx.instructions,
        })
    }
}

impl AstVisitor<'_> for BytecodeGen {
    type Ctx = BytecodeGenCtx;

    fn visit_stmt(&mut self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {
        
    }
}
