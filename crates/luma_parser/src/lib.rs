pub(crate) mod diagnostics;
mod parse_stmt;
mod parse_expr;

use luma_core::ast::{Ast, Scope, Statement, StatementKind};
use luma_diagnostic::{LumaResult, Reporter, ReporterExt};
use luma_lexer::tokens::{PunctuationKind, Token, TokenKind, TokenStream};

use crate::diagnostics::ParserDiagnostic;

pub struct LumaParser<'a> {
    pub(crate) stream: &'a mut TokenStream,
    pub(crate) reporter: Reporter,
}

impl<'a> LumaParser<'a> {
    pub fn new(stream: &'a mut TokenStream, reporter: &Reporter) -> Self {
        Self { 
            stream,
            reporter: reporter.with_name("parser"),
        }
    }

    pub fn parse(&mut self) -> Ast {
        let mut statements: Vec<Statement> = vec![];

        while !self.is_at_end() {
            match self.parse_statement(None) {
                Ok(statement) => {
                    let is_end = statement.kind == StatementKind::EndOfFile;

                    statements.push(statement);

                    if is_end {
                        break;
                    }
                },
                Err(err) => {
                    self.reporter.report(err);
                }
            }
        }

        Ast::new(statements)
    }

    // MARK: Scope
    pub fn consume_scope(&mut self) -> LumaResult<Scope> {
        let (mut span, cursor) = self.consume(TokenKind::Punctuation(PunctuationKind::LeftBrace))?.pos();
        let mut statements = Vec::new();

        let mut had_return = false;
        
        while !self.is_at_end() {
            match self.parse_statement(Some(had_return)) {
                Ok(statement) => {
                    let kind = statement.kind.clone();
                    
                    if had_return {
                        self.reporter.report(self.diagnostic_at(ParserDiagnostic::UnusedStatementDueToEarlyReturn, statement.span, statement.cursor));
                    } else {
                        statements.push(statement);
                    }
                    
                    if self.previous().kind != TokenKind::Punctuation(PunctuationKind::Semicolon) || matches!(kind, StatementKind::Break(_) | StatementKind::Continue(_) | StatementKind::Return(_)) {
                        had_return = true
                    }
                },
                Err(err) => {
                    self.reporter.report(err);
                }
            }

            if let Ok(rbrace) = self.consume(TokenKind::Punctuation(PunctuationKind::RightBrace)) {
                span = span.merge(&rbrace.span);
                break;
            }
        }

        Ok(Scope {
            cursor,
            span,
            statements,
        })
    }

    // MARK: Expect Identifier
    fn expect_lexeme(&mut self, identifier: &str) -> LumaResult<&Token> {
        let current = self.current();
        if current.kind == TokenKind::Identifier && current.lexeme == identifier {
            let current = self.advance();
            return Ok(current);
        }

        Err(self.diagnostic(ParserDiagnostic::ExpectedSpecialKeyword(identifier.to_string())))
    }

    // MARK: Stream funcs
    fn current(&self) -> &Token {
        self.stream.current()
    }

    #[allow(dead_code)]
    fn lookahead(&self) -> &Token {
        self.stream.lookahead_by(1)
    }

    fn advance(&mut self) -> &Token {
        self.stream.advance()
    }

    fn previous(&self) -> &Token {
        self.stream.previous()
    }

    fn is_at_end(&self) -> bool {
        self.stream.is_at_end()
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.current().kind == kind
    }

    fn consume(&mut self, kind: TokenKind) -> LumaResult<&Token> {
        self.expect(kind)?;
        Ok(self.advance())
    }

    fn expect(&mut self, kind: TokenKind) -> LumaResult<&Token> {
        let current = self.current();
        if current.kind == kind {
            Ok(current)
        } else {
            Err(self.diagnostic(ParserDiagnostic::ExpectedToken(kind, current.kind)))
        }
    }
}
