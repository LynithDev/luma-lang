mod _01_type_checking;

pub use _01_type_checking::TypeChecking;

use crate::{AnalyzerStage, aast::AnnotatedAst, stages::analyzer::AnalyzerPass};

pub fn default_aast_passes() -> Vec<Box<dyn AnalyzerPass<AnnotatedAst>>> {
    vec![
        Box::new(TypeChecking),
    ]
}

impl Default for AnalyzerStage<AnnotatedAst> {
    fn default() -> Self {
        Self::with_passes(default_aast_passes())
    }
}


