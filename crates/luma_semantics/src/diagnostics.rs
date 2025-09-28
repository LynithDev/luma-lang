use luma_core::types::TypeKind;
use luma_diagnostic::Diagnostic;

#[derive(Debug, Clone, PartialEq, Diagnostic)]
pub enum AnalyzerDiagnostic {
    #[error("mismatched types: expected `{0}`, found `{1}`")]
    MismatchedTypes(TypeKind, TypeKind),
    #[error("unresolved symbol: `{0}`")]
    UnresolvedSymbol(String),
    #[error("invalid literal value for type `{0}`: `{1}`")]
    InvalidLiteralValue(TypeKind, String),
}