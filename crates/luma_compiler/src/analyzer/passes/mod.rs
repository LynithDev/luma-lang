mod _01_name_declaration;
mod _02_name_resolution;
mod _03_type_inference;

pub use _01_name_declaration::NameDeclaration;
pub use _02_name_resolution::NameResolution;
pub use _03_type_inference::TypeInference;

use crate::analyzer::AnalyzerStage;

pub fn default_passes() -> Vec<Box<dyn AnalyzerStage>> {
    vec![
        Box::new(NameDeclaration),
        Box::new(NameResolution),
    ]
}


