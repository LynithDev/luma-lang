mod _01_scope_identification;
mod _02_name_declaration;
mod _03_name_resolution;
mod _04_type_inference;

pub use _01_scope_identification::ScopeIdentification;
pub use _02_name_declaration::NameDeclaration;
pub use _03_name_resolution::NameResolution;
pub use _04_type_inference::TypeInference;

#[cfg(test)]
pub mod tests;

use crate::stages::analyzer::AnalyzerStage;

pub fn default_passes() -> Vec<Box<dyn AnalyzerStage>> {
    vec![
        Box::new(ScopeIdentification),
        Box::new(NameDeclaration),
        Box::new(NameResolution),
        Box::new(TypeInference),
    ]
}


