use luma_core::ast::TypeKind;
use luma_diagnostic::Diagnostic;

#[derive(Debug, Clone, PartialEq, Diagnostic)]
pub enum AnalyzerDiagnostic {
    #[error("Mismatched types: expected `{0}`, found `{1}`")]
    MismatchedTypes(TypeKind, TypeKind)
}