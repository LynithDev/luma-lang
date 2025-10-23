use crate::{Cursor, Span, SymbolId};

pub mod expressions;
pub mod statements;

pub mod prelude {
    pub use super::expressions::*;
    pub use super::statements::*;

    pub use crate::operators::*;
    pub use crate::types::*;
    pub use crate::visibility::*;

    pub use super::{Ast, ConditionalBranch};
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Ast {
    pub statements: Vec<prelude::Statement>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalBranch {
    pub condition: prelude::Expression,
    pub body: prelude::Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstSymbol {
    pub name: String,
    pub id: Option<SymbolId>,
    pub span: Span,
    pub cursor: Cursor,
}

impl AstSymbol {
    pub fn new(name: String, span: Span, cursor: Cursor) -> Self {
        Self {
            id: None,
            name,
            span,
            cursor,
        }
    }
}
