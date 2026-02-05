#![feature(error_generic_member_access)]

use std::backtrace::{Backtrace, BacktraceStatus};

use luma_core::Span;

pub type CompilerResult<T> = Result<T, LumaError>;

#[derive(thiserror::Error, Debug)]
pub struct LumaError {
    pub kind: Box<dyn std::error::Error>,
    pub span: Option<Span>,
 
    #[backtrace]
    pub backtrace: Backtrace,
}

impl LumaError {
    #[must_use]
    #[inline(always)]
    pub fn spanned<E: std::error::Error + 'static>(kind: E, span: Span) -> Self{
        Self {
            kind: Box::new(kind),
            span: Some(span),
            backtrace: Backtrace::capture(),
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn new<E: std::error::Error + 'static>(kind: E) -> Self {
        Self {
            kind: Box::new(kind),
            span: None,
            backtrace: Backtrace::capture(),
        }
    }
}

impl std::fmt::Display for LumaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(span) = &self.span {
            write!(f, "{} at {}", self.kind, span)?;
        } else {
            write!(f, "{}", self.kind)?;
        }
        
        if self.backtrace.status() == BacktraceStatus::Captured {
            write!(f, "\nBacktrace:\n{}", self.backtrace)?;
        }

        Ok(())
    }
}