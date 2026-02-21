mod expr;
mod operator;
mod stmt;
mod symbol;
mod walker;

pub use expr::*;
pub use operator::*;
pub use stmt::*;
pub use symbol::*;
pub use walker::*;

use luma_core::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotatedAst {
    pub statements: Vec<AnnotStmt>,
    pub span: Span,
}

impl AnnotatedAst {
    #[must_use]
    pub fn new(span: Span, statements: Vec<AnnotStmt>) -> Self {
        Self { span, statements }
    }
}
