mod _01_scope_identification;
mod _02_name_declaration;
mod _03_name_resolution;
mod _04_type_inference;
mod _05_type_solving;
mod _06_type_finalization;

pub use _01_scope_identification::ScopeIdentification;
pub use _02_name_declaration::NameDeclaration;
pub use _03_name_resolution::NameResolution;
pub use _04_type_inference::TypeInference;
pub use _05_type_solving::TypeSolving;
pub use _06_type_finalization::TypeFinalization;

#[cfg(test)]
pub mod tests;

use crate::{AnalyzerStage, ast::Ast, stages::analyzer::AnalyzerPass};

pub fn default_ast_passes() -> Vec<Box<dyn AnalyzerPass<Ast>>> {
    vec![
        Box::new(ScopeIdentification),
        Box::new(NameDeclaration),
        Box::new(NameResolution),
        Box::new(TypeInference),
        Box::new(TypeSolving),
        Box::new(TypeFinalization),
    ]
}

impl Default for AnalyzerStage<Ast> {
    fn default() -> Self {
        Self::with_passes(default_ast_passes())
    }
}


