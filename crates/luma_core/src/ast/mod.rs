mod expr;
mod stmt;
mod symbol;
mod types;
mod walker;

pub use expr::*;
pub use stmt::*;
pub use symbol::*;
pub use types::*;
pub use walker::*;

use crate::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

impl Ast {
    #[must_use]
    pub fn new(span: Span, statements: Vec<Stmt>) -> Self {
        Self { span, statements }
    }
}
