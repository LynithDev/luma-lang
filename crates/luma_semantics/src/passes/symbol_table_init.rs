use luma_core::ast::StatementKind;
use luma_diagnostic::LumaResult;

use crate::AnalyzerContext;

pub struct SymbolTableBuildingPass<'a, 'ctx> {
    ctx: &'a mut AnalyzerContext<'ctx>,
}

impl<'a, 'ctx> SymbolTableBuildingPass<'a, 'ctx> {
    pub fn run(ctx: &'a mut AnalyzerContext<'ctx>) -> LumaResult<()> {
        let mut this = Self {
            ctx
        };

        this.iter()
    }

    fn iter(&mut self) -> LumaResult<()> {
        // for statement in self.ctx.input.ast.statements.iter() {
        //     match statement.kind {
        //         StatementKind::VarDecl(ref decl) => {
        //             let Some(ty) = decl.ty else {
                        
        //             };

        //             ctx.symbol_table.declare(decl.name, decl.ty)
        //         },
        //         _ => todo!("handle {} kind", statement.kind)
        //     }
        // }

        Ok(())
    }

}
