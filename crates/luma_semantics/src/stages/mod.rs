mod name_resolution;
mod ast_lowering;
mod type_inference;

pub use name_resolution::NameResolutionStage;
pub use ast_lowering::AstLoweringStage;
pub use type_inference::TypeInferenceStage;

#[inline]
pub fn get_default_stages() -> Vec<Box<dyn crate::AnalyzerStage>> {
    vec![
        Box::new(NameResolutionStage),
        Box::new(AstLoweringStage),
        Box::new(TypeInferenceStage),
    ]
}