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
    #[error("invalid amount of arguments: expected `{0}`, found `{1}`")]
    InvalidAmountOfArguments(usize, usize),
    #[error("callee is not a function, found `{0}`")]
    CalleeNotFunction(TypeKind),
    #[error("expected type '{0}', found '{1}'")]
    ExpectedTypeFoundType(TypeKind, TypeKind),
}