use luma_core::{ast::{prelude::*, AstSymbol}, Cursor, Span};
use luma_diagnostic::{DiagnosticResult, ReporterExt};
use luma_lexer::tokens::{KeywordKind, LiteralKind, OperatorKind, PunctuationKind, TokenKind};

use crate::{LumaParser, diagnostics::ParserDiagnostic};

impl LumaParser<'_> {
    pub fn parse_statement(&mut self, semicolon: Option<bool>) -> DiagnosticResult<Statement> {
        let result = self.statement();

        let check_semi = if let Some(semicolon) = semicolon {
            semicolon
        } else if let Ok(statement) = &result {
            self.is_semi_required(statement)
        } else {
            true
        };

        if let Some(err) = self
                .consume(TokenKind::Punctuation(PunctuationKind::Semicolon))
                .err() && result.is_ok() && check_semi
        {
            self.reporter.report(err);
        }

        result
    }

    // MARK: Statement
    fn statement(&mut self) -> DiagnosticResult<Statement> {
        let current = self.current();

        match current.kind {
            TokenKind::Keyword(KeywordKind::Function)
            | TokenKind::Keyword(KeywordKind::Var)
            | TokenKind::Keyword(KeywordKind::Class)
            | TokenKind::Keyword(KeywordKind::Public) => self.declaration(),

            TokenKind::Keyword(KeywordKind::Return) => self.stmt_return(),
            TokenKind::Keyword(KeywordKind::Continue) => self.stmt_loop_control(KeywordKind::Continue),
            TokenKind::Keyword(KeywordKind::Break) => self.stmt_loop_control(KeywordKind::Break),
            TokenKind::Keyword(KeywordKind::Import) => self.stmt_import(),
            
            TokenKind::EndOfFile => Ok(Statement {
                kind: StatementKind::EndOfFile,
                span: current.span,
                cursor: current.cursor,
            }),

            _ => self.stmt_expression(),
        }
    }

    // MARK: Declaration
    fn declaration(&mut self) -> DiagnosticResult<Statement> {
        let (mut span, mut cursor) = self.current().pos();
        let visibility = self.get_visibility(&mut span, &mut cursor)?;
        
        let kind = self.current().kind;

        let mut stmt = match kind {
            TokenKind::Keyword(KeywordKind::Function) => self.stmt_function(None),
            TokenKind::Keyword(KeywordKind::Var) => self.stmt_var(),
            TokenKind::Keyword(KeywordKind::Class) => todo!("class decl not implemented"),

            _ => Err(self.diagnostic(
                ParserDiagnostic::UnexpectedToken(kind),
            )),
        }?;

        match &mut stmt.kind {
            StatementKind::VarDecl(decl) => {
                decl.visibility = visibility;
            }
            StatementKind::FuncDecl(decl) => {
                decl.visibility = visibility;
            }
            StatementKind::ClassDecl(decl) => {
                decl.visibility = visibility;
            }
            _ => unreachable!()
        }

        Ok(stmt)
    }

    // MARK: Visibility
    fn get_visibility(&mut self, span: &mut Span, cursor: &mut Cursor) -> DiagnosticResult<Visibility> {
        let Ok((token_span, token_cursor)) = self.consume(TokenKind::Keyword(KeywordKind::Public)).map(|t| t.pos()) else {
            return Ok(Visibility::default());
        };

        *span = span.merge(&token_span);
        *cursor = token_cursor;

        // Get the visibility scope
        Ok(if self
            .consume(TokenKind::Punctuation(PunctuationKind::LeftParen))
            .is_ok()
        {
            let scope_token = self.current().to_owned();

            let visibility = match scope_token.kind {
                TokenKind::Keyword(KeywordKind::This) => {
                    Visibility::Private
                }
                TokenKind::Identifier => match Visibility::scoped(&scope_token.lexeme) {
                    Some(scope) => {
                        scope
                    }
                    None => {
                        self.reporter.report(self.diagnostic(
                            ParserDiagnostic::InvalidVisibilityScope(scope_token.lexeme.clone()),
                        ));
                        Visibility::default()
                    }
                },
                _ => {
                    self.reporter.report(self.diagnostic(ParserDiagnostic::UnexpectedToken(scope_token.kind)));
                    Visibility::default()
                },
            };

            // skip the scope token
            self.advance();

            // Close visibility scope
            self.consume(TokenKind::Punctuation(PunctuationKind::RightParen))?;

            *span = span.merge(&scope_token.span);
            visibility
        } else {
            Visibility::default()
        })
    }

    // MARK: Function
    fn stmt_function(&mut self, decl_only: Option<bool>) -> DiagnosticResult<Statement> {
        // Consume the function token
        let (mut span, cursor) = self
            .consume(TokenKind::Keyword(KeywordKind::Function))?
            .pos();

        // consume the identifier
        let identifier = self.consume(TokenKind::Identifier)?.clone();

        // consume the parameters
        let _ = self.consume(TokenKind::Punctuation(PunctuationKind::LeftParen))?;
        let mut params: Vec<Parameter> = Vec::new();

        loop {
            if let Ok(rparen) = self.consume(TokenKind::Punctuation(PunctuationKind::RightParen)) {
                span = span.merge(&rparen.span);
                break;
            }

            if !params.is_empty() {
                self.consume(TokenKind::Punctuation(PunctuationKind::Comma))?;
            }

            let (span, cursor) = self.current().pos();

            // consume mut
            let is_mut = self.consume(TokenKind::Keyword(KeywordKind::Mut)).is_ok();

            // consume identifier
            let identifier = self.consume(TokenKind::Identifier)?.clone();

            // consume type
            self.expect(TokenKind::Punctuation(PunctuationKind::Colon))?;
            let Some(ty) = self.parse_type()? else {
                return Err(
                    self.diagnostic(ParserDiagnostic::MissingTypeAnnotation(identifier.lexeme))
                );
            };

            params.push(Parameter {
                mutable: is_mut,
                symbol: AstSymbol::new(identifier.lexeme, identifier.span, identifier.cursor),
                span: span.merge(&ty.span),
                cursor,
                ty,
            });
        }

        // Consume the function return type if there is one
        let return_type: Option<Type> =
            if self.check(TokenKind::Punctuation(PunctuationKind::Colon)) {
                let Some(ty) = self.parse_type()? else {
                    return Err(
                        self.diagnostic(ParserDiagnostic::MissingTypeAnnotation(identifier.lexeme))
                    );
                };

                span = span.merge(&ty.span);

                Some(ty)
            } else {
                None
            };

        // consume body
        let body: Option<Box<Expression>> = if decl_only.unwrap_or(false) {
            self.consume(TokenKind::Punctuation(PunctuationKind::Semicolon))?;
            None
        } else {
            let kind = self.current().kind;
            match kind {
                TokenKind::Punctuation(PunctuationKind::LeftBrace) => {
                    let scope = self.parse_expression()?;
                    Some(Box::new(scope))
                }
                TokenKind::Operator(OperatorKind::Equals) => {
                    self.advance();
                    let expr: Box<Expression> = Box::new(self.parse_expression()?);
                    Some(expr)
                }
                TokenKind::Punctuation(PunctuationKind::Semicolon) => {
                    self.advance();
                    return Err(self.diagnostic(ParserDiagnostic::MissingFunctionBody));
                }
                _ => {
                    return Err(self.diagnostic(ParserDiagnostic::UnexpectedToken(kind)));
                }
            }
        };

        Ok(Statement {
            cursor,
            span,
            kind: StatementKind::FuncDecl(FuncDecl {
                visibility: Visibility::default(),
                symbol: AstSymbol::new(
                    identifier.lexeme.clone(),
                    identifier.span,
                    identifier.cursor,
                ),
                parameters: params,
                return_type,
                body,
            }),
        })
    }

    // MARK: Expression
    fn stmt_expression(&mut self) -> DiagnosticResult<Statement> {
        let expr = self.parse_expression()?;
        let current = self.current();

        Ok(Statement {
            span: expr.span.merge(&current.span),
            cursor: current.cursor,
            kind: StatementKind::Expression { inner: expr },
        })
    }

    // MARK: Var
    fn stmt_var(&mut self) -> DiagnosticResult<Statement> {
        // Consume the var token
        let (span, cursor) = self.consume(TokenKind::Keyword(KeywordKind::Var))?.pos();

        // Consume the mut token if it exists
        let is_mut = self.consume(TokenKind::Keyword(KeywordKind::Mut)).is_ok();

        // Get the identifier token and its name
        let identifier = self.consume(TokenKind::Identifier)?.clone();

        // Optionally get the type
        let ty: Option<Type> = self.parse_type()?;

        // Optionally get the value
        let value: Option<Box<Expression>> = if self
            .consume(TokenKind::Operator(OperatorKind::Equals))
            .is_ok()
        {
            let expr = self.parse_expression()?;
            Some(Box::new(expr))
        } else {
            None
        };

        Ok(Statement {
            cursor,
            span: span.merge_all(&[
                Some(identifier.span),
                ty.as_ref().map(|ty| ty.span),
                value.as_ref().map(|expr| expr.span),
            ]),
            kind: StatementKind::VarDecl(VarDecl {
                visibility: Visibility::default(),
                mutable: is_mut,
                symbol: AstSymbol::new(
                    identifier.lexeme.clone(),
                    identifier.span,
                    identifier.cursor,
                ),
                ty,
                value,
            }),
        })
    }

    // MARK: Return
    fn stmt_return(&mut self) -> DiagnosticResult<Statement> {
        // Consume the return token
        let (mut span, cursor) = self.consume(TokenKind::Keyword(KeywordKind::Return))?.pos();

        // Optionally get the value
        let value: Option<Expression> =
            if self.is_at_end() || self.check(TokenKind::Punctuation(PunctuationKind::Semicolon)) {
                None
            } else {
                let expr = self.parse_expression()?;
                span = span.merge(&expr.span);
                Some(expr)
            };

        Ok(Statement {
            cursor,
            span,
            kind: StatementKind::Return { value },
        })
    }

    // MARK: Break / Continue
    fn stmt_loop_control(&mut self, kind: KeywordKind) -> DiagnosticResult<Statement> {
        let (mut span, cursor) = self.consume(TokenKind::Keyword(kind))?.pos();

        let label = if self
            .consume(TokenKind::Punctuation(PunctuationKind::Colon))
            .is_ok()
        {
            let identifier = self.consume(TokenKind::Identifier)?;
            span = span.merge(&identifier.span);

            Some(AstSymbol::new(
                identifier.lexeme.clone(),
                identifier.span,
                identifier.cursor,
            ))
        } else {
            None
        };

        Ok(Statement {
            cursor,
            span,
            kind: match kind {
                KeywordKind::Continue => StatementKind::Continue { label },
                KeywordKind::Break => StatementKind::Break { label },
                _ => unreachable!("{kind} is not a valid loop control"),
            },
        })
    }

    // MARK: Import
    fn stmt_import(&mut self) -> DiagnosticResult<Statement> {
        // Consume the import token
        let (span, cursor) = self.consume(TokenKind::Keyword(KeywordKind::Import))?.pos();

        // Determine the import property kind
        let current = self.current();
        let property_kind: ImportPropertyKind = match current.kind {
            TokenKind::Operator(OperatorKind::Asterisk) => {
                self.consume(TokenKind::Operator(OperatorKind::Asterisk))?;

                self.expect_lexeme("as")?;

                let identifier = self.consume(TokenKind::Identifier)?.lexeme.clone();

                self.expect_lexeme("from")?;

                ImportPropertyKind::All(identifier)
            }
            TokenKind::Punctuation(PunctuationKind::LeftBrace) => {
                self.consume(TokenKind::Punctuation(PunctuationKind::LeftBrace))?;

                // Consume the identifier token
                let mut imports: Vec<Renameable> = Vec::new();

                while !self.check(TokenKind::Punctuation(PunctuationKind::RightBrace)) {
                    if !imports.is_empty() {
                        self.consume(TokenKind::Punctuation(PunctuationKind::Comma))?;
                    }

                    let identifier = self.consume(TokenKind::Identifier)?.lexeme.clone();

                    if self.expect_lexeme("as").is_ok() {
                        let name = self.consume(TokenKind::Identifier)?.lexeme.clone();
                        imports.push(Renameable::Renamed(identifier, name));
                    } else {
                        imports.push(Renameable::Normally(identifier));
                    }
                }

                self.consume(TokenKind::Punctuation(PunctuationKind::RightBrace))?;

                let _ = self.expect_lexeme("from")?;

                ImportPropertyKind::Individual(imports)
            }
            TokenKind::Literal(LiteralKind::String) => {
                // we handle the lib name later anyways
                ImportPropertyKind::None
            }
            _ => {
                return Err(self.diagnostic(ParserDiagnostic::UnexpectedToken(current.kind)));
            }
        };

        // Get the library name
        let lib_name = self
            .consume(TokenKind::Literal(LiteralKind::String))?
            .lexeme
            .clone();

        Ok(Statement {
            cursor,
            span,
            kind: StatementKind::Import {
                kind: property_kind,
                path: lib_name,
            },
        })
    }

    // MARK: Parse Type
    fn parse_type(&mut self) -> DiagnosticResult<Option<Type>> {
        Ok(
            if self
                .consume(TokenKind::Punctuation(PunctuationKind::Colon))
                .is_ok()
            {
                let identifier = self.consume(TokenKind::Identifier)?;
                Some(Type {
                    kind: TypeKind::from(identifier.lexeme.as_str()),
                    cursor: identifier.cursor,
                    span: identifier.span,
                })
            } else {
                None
            },
        )
    }
}
