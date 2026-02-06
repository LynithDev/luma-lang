use std::backtrace::{Backtrace, BacktraceStatus};

use luma_core::Span;

#[derive(thiserror::Error, Debug)]
pub struct LumaError {
    #[source]
    pub source: Box<dyn std::error::Error>,

    pub span: Option<Span>,

    pub compiler_location: ErrorSource,

    #[backtrace]
    pub backtrace: Backtrace,
}

#[derive(Debug)]
pub struct ErrorSource {
    pub line: u32,
    pub column: u32,
    pub file: &'static str,
}

#[macro_export]
macro_rules! error {
    ($kind:expr$(,)?) => {
        error!(@internal $kind, None)
    };

    ($kind:expr, $span:expr$(,)?) => {
        error!(@internal $kind, Some($span))
    };

    (@internal $kind:expr, $span_opt:expr) => {
        $crate::LumaError {
            source: Box::new($kind),
            span: $span_opt,
            compiler_location: $crate::ErrorSource {
                line: line!(),
                column: column!(),
                file: file!(),
            },
            backtrace: std::backtrace::Backtrace::capture(),
        }
    };
}

impl std::fmt::Display for LumaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] ", self.compiler_location)?;

        if let Some(span) = &self.span {
            write!(f, "{} at bytes {}", self.source, span)?;
        } else {
            write!(f, "{}", self.source)?;
        }
        
        if self.backtrace.status() == BacktraceStatus::Captured {
            write!(f, "\n{}", self.backtrace)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}
