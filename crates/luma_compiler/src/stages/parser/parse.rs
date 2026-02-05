use crate::ast::*;
use luma_core::Span;
use luma_diagnostic::{CompilerResult, LumaError};

use crate::stages::{
    lexer::{Token, TokenKind},
    parser::{ctx::ParserContext, error::ParserErrorKind},
};

pub struct TokenParser<'a> {
    pub(super) tokens: &'a [Token],

    pub(super) index: usize,
    pub(super) ctx: ParserContext,
}

impl TokenParser<'_> {
    pub(super) fn new<'a>(tokens: &'a [Token]) -> TokenParser<'a> {
        TokenParser {
            tokens,
            index: 0,

            ctx: ParserContext::default(),
        }
    }

    /// The main parsing loop.
    pub(super) fn parse_tokens(mut self, errors: &mut Vec<LumaError>) -> Ast {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            match self.parse_statement(None) {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    errors.push(err);

                    self.synchronize();
                }
            }
        }

        Ast {
            span: *Span::default().maybe_merge(&statements.last().map(|node| node.span)),
            statements,
        }
    }

    /// Synchronize the parser after an error to the next statement boundary.
    fn synchronize(&mut self) {
        while !self.is_at_end() {
            let token = self.current();

            if token.kind == TokenKind::Semicolon {
                self.advance();
                return;
            }

            self.advance();
        }
    }

    /// Get the current token without advancing the parser.
    #[must_use]
    pub(super) fn current(&self) -> Token {
        self.tokens[self.index].clone()
    }

    /// Peek at the next token without advancing the parser.
    #[must_use]
    pub(super) fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    /// Check if the next token matches the expected kind.
    #[must_use]
    pub(super) fn check_next(&self, expected: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().is_some_and(|token| token.kind == expected)
    }

    /// Advance the parser to the next token and return it.
    ///
    /// If already at the end, stays at the end.
    pub(super) fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.index += 1;
        }

        self.current()
    }

    /// Assert that the current token is of a specific kind, otherwise return an error.
    pub(super) fn assert(&self, expected: TokenKind) -> CompilerResult<Token> {
        let current = self.current();

        if current.kind != expected {
            return Err(LumaError::spanned(
                ParserErrorKind::ExpectedToken {
                    expected,
                    found: current.kind.clone(),
                },
                current.span,
            ));
        }

        Ok(current)
    }

    /// Check if the current token matches the expected kind.
    #[must_use]
    pub(super) fn check(&self, expected: TokenKind) -> bool {
        self.current().kind == expected
    }

    /// Consumes the current token if it matches the expected kind, otherwise return an error.
    pub(super) fn consume(&mut self, expected: TokenKind) -> CompilerResult<Token> {
        let token = self.assert(expected)?;
        self.advance();
        Ok(token)
    }

    /// Check if the parser has reached the end of the token stream.
    #[must_use]
    pub(super) fn is_at_end(&self) -> bool {
        self.index + 1 >= self.tokens.len()
    }
}
