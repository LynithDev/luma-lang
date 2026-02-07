use crate::ast::ExprKind;
use luma_diagnostic::define_diagnostics;

use crate::stages::lexer::TokenKind;

define_diagnostics! {
    pub enum ParserError {
        #[Error("unexpected token", "expected '{expected}', found '{found}'")]
        ExpectedToken {
            expected: TokenKind,
            found: TokenKind,
        },
        #[Error("unexpected token", "found '{found}'")]
        UnexpectedToken {
            found: TokenKind,
        },
        #[Error("unexpected end of input")]
        UnexpectedEndOfInput,
        #[Error("invalid char literal", "found '{lexeme}'")]
        InvalidCharLiteral {
            lexeme: String
        },
        #[Error("invalid float literal", "found '{lexeme}': {source}")]
        InvalidFloatLiteral {
            lexeme: String,
            source: String,
        },
        #[Error("invalid integer literal", "found '{lexeme}': {source}")]
        InvalidIntegerLiteral {
            lexeme: String,
            source: String,
        },
        #[Error("invalid boolean literal", "found '{lexeme}'")]
        InvalidBooleanLiteral {
            lexeme: String,
        },
        #[Error("invalid suffix for literal", "found '{suffix}'")]
        InvalidLiteralSuffix {
            suffix: String,
        },
        #[Error("invalid struct literal target", "expected identifier found '{found}'")]
        InvalidStructLiteralTarget {
            found: ExprKind,
        },
        #[Error("invalid visibility specifier", "found '{ident}'")]
        InvalidVisibility {
            ident: String,
        },
        #[Error("missing body for function declaration")]
        MissingFunctionBody,
        #[Error("invalid type", "found '{type_name}'")]
        InvalidType {
            type_name: String,
        },
    }
}