use luma_core::{Cursor, Span};
use luma_diagnostic::{Diagnostic, DiagnosticReport, ReporterExt};

use crate::LumaLexer;

#[derive(Debug, Clone, PartialEq, Diagnostic)]
pub enum LexerDiagnostic {
    #[warning("unexpected character '{0}'")]
    UnexpectedCharacter(char),

    #[error("unexpected radix identifier '{0}'")]
    UnexpectedRadixIdentifier(char),

    #[error("unterminated string")]
    #[note("strings must begin and end with ' or \"")]
    UnterminatedString,
}

impl ReporterExt for LumaLexer<'_> {
    type Message = LexerDiagnostic;

    fn diagnostic(&self, message: Self::Message) -> DiagnosticReport {
        DiagnosticReport {
            message: Box::new(message),
            span: Span {
                start: self.start,
                end: self.start + self.length,
            },
            cursor: Cursor {
                line: self.line,
                column: self.column,
            }
        }
    }
}
