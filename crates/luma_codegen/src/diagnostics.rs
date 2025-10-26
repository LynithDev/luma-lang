use luma_core::SymbolId;
use luma_diagnostic::Diagnostic;
use luma_semantics::hir::prelude::HirExpressionKind;

#[derive(Debug, Clone, PartialEq, Diagnostic)]
pub enum CodegenDiagnostic {
    #[error("expected expression to be of '{0}' found '{1}'")]
    ExpectedExpression(HirExpressionKind, HirExpressionKind),
    #[error("unable to capture non-existent upvalue for symbol id '{0}'")]
    UnableToCaptureUpvalue(SymbolId),
}
