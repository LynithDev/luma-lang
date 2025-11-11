use luma_core::{Cursor, Span};

#[cfg(feature = "reporter")]
mod reporter;

#[cfg(feature = "reporter")]
pub use reporter::*;

mod diagnostic_store;
pub use diagnostic_store::*;
pub use luma_macros::Diagnostic;

pub type DiagnosticResult<T> = Result<T, DiagnosticReport>;

#[derive(Debug)]
pub struct DiagnosticReport {
    pub message: Box<dyn DiagnosticMessage>,
    pub cursor: Cursor,
    pub span: Span,
}

pub trait DiagnosticMessage: std::fmt::Display + std::fmt::Debug + 'static {
    fn kind(&self) -> DiagnosticKind;
    
    fn note(&self) -> Option<String> {
        None
    }

    fn message(&self) -> String {
        self.to_string()
    }
}

#[derive(luma_macros::Display, Debug, Clone, PartialEq, Eq, Hash)]
#[display(case = "snake_case")]
pub enum DiagnosticKind {
    Warning,
    Error,
}

pub trait ReporterExt {
    type Message: DiagnosticMessage;

    fn diagnostic(&self, message: Self::Message) -> DiagnosticReport;

    fn diagnostic_at(&self, message: Self::Message, span: Span, cursor: Cursor) -> DiagnosticReport {
        DiagnosticReport {
            message: Box::new(message),
            span,
            cursor,
        }
    }
}

