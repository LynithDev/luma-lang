use crate::ast::ExprKind;
use thiserror::Error;

use crate::stages::lexer::TokenKind;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ParserErrorKind {
    #[error("expected token '{expected}', found '{found}'")]
    ExpectedToken {
        expected: TokenKind,
        found: TokenKind,
    },
    #[error("unexpected token '{found}'")]
    UnexpectedToken {
        found: TokenKind,
    },
    #[error("unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("char literal must contain a single character, found '{lexeme}'")]
    InvalidCharLiteral {
        lexeme: String
    },
    #[error("invalid float literal '{lexeme}': {source}")]
    InvalidFloatLiteral {
        lexeme: String,
        #[source]
        source: std::num::ParseFloatError,
    },
    #[error("invalid integer literal '{lexeme}': {source}")]
    InvalidIntegerLiteral {
        lexeme: String,
        #[source]
        source: std::num::ParseIntError,
    },
    #[error("invalid boolean literal: '{lexeme}'")]
    InvalidBooleanLiteral {
        lexeme: String,
    },
    #[error("invalid suffix for literal: '{suffix}'")]
    InvalidLiteralSuffix {
        suffix: String,
    },
    #[error("invalid struct literal target, expected identifier found '{found}'")]
    InvalidStructLiteralTarget {
        found: ExprKind,
    },
    #[error("invalid visibility specifier '{ident}'")]
    InvalidVisibility {
        ident: String,
    },
    #[error("missing body in function declaration")]
    MissingFunctionBody,
    #[error("invalid type '{type_name}'")]
    InvalidType {
        type_name: String,
    },
}