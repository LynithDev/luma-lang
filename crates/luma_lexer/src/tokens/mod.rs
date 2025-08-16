use crate::{Cursor, Span};

mod kind;
mod stream;
mod convert;

pub use kind::*;
pub use stream::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub cursor: Cursor,
    pub span: Span,
}

impl Token {
    pub fn pos(&self) -> (Span, Cursor) {
        (self.span, self.cursor)
    }
}

