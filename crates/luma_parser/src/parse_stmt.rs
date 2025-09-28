use luma_diagnostic::{DiagnosticKind, DiagnosticResult, ReporterExt};
use luma_lexer::tokens::{KeywordKind, LiteralKind, OperatorKind, PunctuationKind, TokenKind};
use luma_core::ast::{prelude::*, AstSymbol};

use crate::{diagnostics::ParserDiagnostic, LumaParser};

impl LumaParser<'_> {

    pub fn parse_statement(&mut self, semicolon: Option<bool>) -> DiagnosticResult<Statement> {
        self.declaration(semicolon)
    }

    // MARK: Declaration
    fn declaration(&mut self, semicolon: Option<bool>) -> DiagnosticResult<Statement> {
        let current = self.current();

        match current.kind {
            TokenKind::EndOfFile => {
                Ok(Statement {
                    kind: StatementKind::EndOfFile,
                    span: current.span,
                    cursor: current.cursor,
                })
            },

            TokenKind::Keyword(kind) => match kind {
                KeywordKind::Public => self.stmt_public(),
                KeywordKind::Function => self.stmt_function(None),

                _ => self.statement(semicolon),
            }

            _ => self.statement(semicolon),
        }
    }

    // MARK: Statement
    fn statement(&mut self, semicolon: Option<bool>) -> DiagnosticResult<Statement> {
        let old_error_count = self.reporter.diagnostic_count(DiagnosticKind::Error);

        let current= self.current();
        let result = match current.kind {
            TokenKind::Keyword(kind) => match kind {
                KeywordKind::Var => self.stmt_var(),
                KeywordKind::Return => self.stmt_return(),
                KeywordKind::Continue => self.stmt_loop_control(KeywordKind::Continue),
                KeywordKind::Break => self.stmt_loop_control(KeywordKind::Break),
                KeywordKind::Import => self.stmt_import(),

                _ => todo!("handle other keywords: {kind}"),
            },

            _ => self.stmt_expression(),
        };

        // Check for semicolon
        if let Some(err) = self.consume(TokenKind::Punctuation(PunctuationKind::Semicolon)).err() {
            let new_error_count = self.reporter.diagnostic_count(DiagnosticKind::Error);

            if new_error_count <= old_error_count && semicolon.unwrap_or(true) {
                self.reporter.report(err);
            }
        }

        result
    }

    // MARK: Visibility
    fn stmt_public(&mut self) -> DiagnosticResult<Statement> {
        // Consume the pub token
        let (span, cursor) = self.consume(TokenKind::Keyword(KeywordKind::Public))?.pos();

        // Get the visibility scope
        let mut visibility = Visibility::Public;

        if self.consume(TokenKind::Punctuation(PunctuationKind::LeftParen)).is_ok() {
            let next = self.advance().to_owned();

            match next.kind {
                TokenKind::Keyword(KeywordKind::This) => {
                    visibility = Visibility::Private;
                },
                TokenKind::Identifier => {
                    match Visibility::scoped(&next.lexeme) {
                        Some(scope) => {
                            visibility = scope;
                        },
                        None => {
                            self.reporter.report(self.diagnostic(ParserDiagnostic::InvalidVisibilityScope(next.lexeme.clone())));
                        },
                    }
                },
                _ => return Err(self.diagnostic(ParserDiagnostic::UnexpectedToken(next.kind)))
            };

            // Close visibility scope
            self.consume(TokenKind::Punctuation(PunctuationKind::RightParen))?;
        }

        let mut statement = self.parse_statement(None)?;

        match &mut statement.kind {
            StatementKind::VarDecl(decl) => {
                decl.visibility = visibility;
            },
            StatementKind::FuncDecl(decl) => {
                decl.visibility = visibility;
            },
            StatementKind::ClassDecl(decl) => {
                decl.visibility = visibility;
            },
            _ => return Err(self.diagnostic(ParserDiagnostic::UnsupportedVisibilityStatement(Box::new(statement.kind)))),
        }

        statement.cursor = cursor;
        statement.span = span.merge(&statement.span);

        Ok(statement)
    }

    // MARK: Function
    fn stmt_function(&mut self, decl_only: Option<bool>) -> DiagnosticResult<Statement> {
        // Consume the function token
        let (mut span, cursor) = self.consume(TokenKind::Keyword(KeywordKind::Function))?.pos();

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
                return Err(self.diagnostic(ParserDiagnostic::MissingTypeAnnotation(identifier.lexeme)));
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
        let return_type: Option<Type> = if self.check(TokenKind::Punctuation(PunctuationKind::Colon)) {
            let Some(ty) = self.parse_type()? else {
                return Err(self.diagnostic(ParserDiagnostic::MissingTypeAnnotation(identifier.lexeme)));
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
                },
                TokenKind::Operator(OperatorKind::Equals) => {
                    self.advance();
                    let expr: Box<Expression> = Box::new(self.parse_expression()?);
                    self.consume(TokenKind::Punctuation(PunctuationKind::Semicolon))?;
                    Some(expr)
                }
                TokenKind::Punctuation(PunctuationKind::Semicolon) => {
                    self.advance();
                    return Err(self.diagnostic(ParserDiagnostic::MissingFunctionBody));
                },
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
                symbol: AstSymbol::new(identifier.lexeme.clone(), identifier.span, identifier.cursor),
                parameters: params,
                return_type,
                body,
            }),
        })
    }

    // MARK: Expression
    fn stmt_expression(&mut self) -> DiagnosticResult<Statement> {
        self.parse_expression().map(|expr| {
            let current = self.current();
            Statement {
                span: expr.span.merge(&current.span),
                cursor: current.cursor,
                kind: StatementKind::Expression {
                    inner: expr
                },
            }
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
        let value: Option<Box<Expression>> = if self.consume(TokenKind::Operator(OperatorKind::Equals)).is_ok() {
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
                symbol: AstSymbol::new(identifier.lexeme.clone(), identifier.span, identifier.cursor), 
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
        let value: Option<Box<Expression>> = if self.is_at_end() || self.check(TokenKind::Punctuation(PunctuationKind::Semicolon)) {
            None
        } else {
            let expr = self.parse_expression()?;
            span = span.merge(&expr.span);
            Some(Box::new(expr))
        };

        Ok(Statement {
            cursor,
            span,
            kind: StatementKind::Return {
                value
            },
        })
    }

    // MARK: Break / Continue
    fn stmt_loop_control(&mut self, kind: KeywordKind) -> DiagnosticResult<Statement> {
        let (mut span, cursor) = self.consume(TokenKind::Keyword(kind))?.pos();

        let label = if self.consume(TokenKind::Punctuation(PunctuationKind::Colon)).is_ok() {
            let identifier = self.consume(TokenKind::Identifier)?;
            span = span.merge(&identifier.span);
            
            Some(AstSymbol::new(identifier.lexeme.clone(), identifier.span, identifier.cursor))
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
            },
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
            },
            TokenKind::Literal(LiteralKind::String) => {
                // we handle the lib name later anyways
                ImportPropertyKind::None
            }
            _ => {
                return Err(self.diagnostic(ParserDiagnostic::UnexpectedToken(current.kind)));
            }
        };

        // Get the library name
        let lib_name = self.consume(TokenKind::Literal(LiteralKind::String))?.lexeme.clone();

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
        Ok(if self.consume(TokenKind::Punctuation(PunctuationKind::Colon)).is_ok() {
            let identifier = self.consume(TokenKind::Identifier)?;
            Some(Type {
                kind: TypeKind::from(identifier.lexeme.as_str()),
                cursor: identifier.cursor,
                span: identifier.span,
            })
        } else {
            None
        })
    }

}