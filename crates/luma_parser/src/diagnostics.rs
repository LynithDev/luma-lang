use luma_core::ast::ExpressionKind;
use luma_diagnostic::{Diagnostic, DiagnosticReport, ReporterExt};
use luma_lexer::tokens::TokenKind;

use crate::{LumaParser, StatementKind};

#[derive(Debug, Clone, PartialEq, Diagnostic)]
pub enum ParserDiagnostic {
    #[error("unexpected end")]
    UnexpectedEnd,
    
    #[error("expected token '{0}', found '{1}'")]
    ExpectedToken(TokenKind, TokenKind),

    #[error("unexpected expression '{0}'")]
    UnexpectedExpression(ExpressionKind),

    #[error("unexpected token '{0}'")]
    UnexpectedToken(TokenKind),

    #[error("invalid visibility scope '{0}'. expected 'inherit', 'module', 'this' or empty")]
    InvalidVisibilityScope(String),

    #[error("statement of kind '{0}' does not support visibility modifiers")]
    UnsupportedVisibilityStatement(Box<StatementKind>),

    #[error("invalid number literal '{0}'")]
    InvalidNumberLiteral(String),

    #[error("variable '{0}' does not have a declared type or value")]
    VariableCannotInfer(String),

    #[error("left-hand side of assignment must be a variable, found '{0}'")]
    InvalidLeftHandSide(Box<ExpressionKind>),

    #[warning("unused statement due to early return")]
    UnusedStatementDueToEarlyReturn,

    #[error("missing type annotation for '{0}'")]
    MissingTypeAnnotation(String),

    #[error("expected special keyword '{0}'")]
    ExpectedSpecialKeyword(String),
    
    #[error("missing function body")]
    MissingFunctionBody,
}

impl ReporterExt for LumaParser<'_> {
    type Message = ParserDiagnostic;

    fn diagnostic(&self, message: Self::Message) -> DiagnosticReport {
        let current = self.current();

        DiagnosticReport {
            message: Box::new(message),
            span: current.span,
            cursor: current.cursor,
        }
    }
}
