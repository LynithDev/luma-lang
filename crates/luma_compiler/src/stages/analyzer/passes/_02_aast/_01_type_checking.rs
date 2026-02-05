use crate::aast::*;

use crate::stages::analyzer::{AnalyzerContext, AnalyzerPass};

pub struct TypeChecking;

impl AnalyzerPass<AnnotatedAst> for TypeChecking {
    fn name(&self) -> String {
        String::from("type_checking")
    }

    fn analyze(&self, ctx: &mut AnalyzerContext, input: &mut AnnotatedAst) {
        let _ = self.traverse(ctx, input);
    }
}

impl AnnotAstVisitor<'_> for TypeChecking {
    type Ctx = AnalyzerContext;
}