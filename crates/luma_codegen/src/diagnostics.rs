use luma_diagnostic::Diagnostic;
use luma_semantics::hir::prelude::HirExpressionKind;

#[derive(Debug, Clone, PartialEq, Diagnostic)]
pub enum CodeGenDiagnostic {
    #[error("expected expression to be of '{0}' found '{1}'")]
    ExpectedExpression(HirExpressionKind, HirExpressionKind),
}
