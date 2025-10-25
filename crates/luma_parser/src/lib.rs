pub(crate) mod diagnostics;
mod parse_expr;
mod parse_stmt;

use luma_core::ast::prelude::*;
use luma_diagnostic::{DiagnosticResult, Reporter, ReporterExt};
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
        let mut ast = Ast::new();

        while !self.is_at_end() {
            match self.parse_statement(None) {
                Ok(statement) => {
                    let is_end = statement.kind == StatementKind::EndOfFile;

                    ast.statements.push(statement);

                    if is_end {
                        break;
                    }
                }
                Err(err) => {
                    self.reporter.report(err);
                }
            }
        }

        ast
    }

    // MARK: Scope
    pub fn consume_scope(&mut self) -> DiagnosticResult<Expression> {
        let (mut span, cursor) = self
            .consume(TokenKind::Punctuation(PunctuationKind::LeftBrace))?
            .pos();

        let mut statements: Vec<Statement> = Vec::new();

        let mut value: Option<Box<Expression>> = None;
        let mut had_return = false;

        while !self.is_at_end() {
            if let Ok(rbrace) = self.consume(TokenKind::Punctuation(PunctuationKind::RightBrace)) {
                span = span.merge(&rbrace.span);
                break;
            }

            match self.parse_statement(Some(had_return)) {
                Ok(statement) => {
                    let kind = statement.kind.clone();

                    let is_implicit_return = self.is_semi_required(&statement)
                        && self.previous().kind != TokenKind::Punctuation(PunctuationKind::Semicolon);

                    let is_return = matches!(
                        kind,
                        StatementKind::Break { .. }
                            | StatementKind::Continue { .. }
                            | StatementKind::Return { .. }
                    ) || is_implicit_return;

                    if is_return {
                        had_return = true;
                    }

                    if is_implicit_return {
                        if let StatementKind::Expression { inner } = kind {
                            value = Some(Box::new(inner));
                        }
                    } else {
                        statements.push(statement);
                    }
                }
                Err(err) => {
                    self.reporter.report(err);
                }
            }
        }

        Ok(Expression {
            cursor,
            span,
            kind: ExpressionKind::Scope { 
                statements,
                block_value: value
            },
        })
    }

    // MARK: Semi check
    pub fn is_semi_required(&mut self, statement: &Statement) -> bool {
        match &statement.kind {
            StatementKind::Expression { inner } => !matches!(
                &inner.kind, 
                    // we don't need a semicolon if the if expression is a "statement expression"
                    ExpressionKind::If { .. }
                    | ExpressionKind::Scope { block_value: None, .. }
            ),

            StatementKind::FuncDecl(decl) => {
                if decl.body.is_none() {
                    true
                } else {
                    // we only don't need a semicolon if the body is a scope
                    !matches!(&decl.body.as_ref().map(|b| &b.kind), Some(ExpressionKind::Scope { .. }))
                }
            },

            StatementKind::EndOfFile => false,
            _ => true,
        }
    }

    // MARK: Condition branch
    fn parse_conditional_branch(&mut self) -> DiagnosticResult<ConditionalBranch> {
        // parse condition
        let condition = self.parse_expression()?;

        // parse body
        self.expect(TokenKind::Punctuation(PunctuationKind::LeftBrace))?;

        let body = self.parse_expression()?;

        Ok(ConditionalBranch { condition, body })
    }

    // MARK: Expect Identifier
    fn expect_lexeme(&mut self, identifier: &str) -> DiagnosticResult<&Token> {
        let current = self.current();
        if current.kind == TokenKind::Identifier && current.lexeme == identifier {
            let current = self.advance();
            return Ok(current);
        }

        Err(self.diagnostic(ParserDiagnostic::ExpectedSpecialKeyword(
            identifier.to_string(),
        )))
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
        self.stream.is_at_end() || self.current().kind == TokenKind::EndOfFile
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.current().kind == kind
    }

    fn consume(&mut self, kind: TokenKind) -> DiagnosticResult<&Token> {
        self.expect(kind)?;
        Ok(self.advance())
    }

    fn expect(&mut self, kind: TokenKind) -> DiagnosticResult<&Token> {
        let current = self.current();
        if current.kind == kind {
            Ok(current)
        } else {
            Err(self.diagnostic(ParserDiagnostic::ExpectedToken(kind, current.kind)))
        }
    }
}
