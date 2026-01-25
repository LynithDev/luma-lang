use luma_core::{Operator, Spanned, ast::*};
use luma_diagnostic::{CompilerResult, LumaError};

use crate::{
    lexer::{Token, TokenKind},
    parser::{context::ParserContext, error::ParserErrorKind},
};

impl ParserContext<'_> {
    // MARK: Expression
    /// Parses an expression
    ///
    /// Ascends to [`Parser::expr_assign`]
    pub fn parse_expression(&mut self) -> CompilerResult<Expr> {
        self.expr_assign()
    }

    // MARK: Assign
    /// Parses assignment expressions
    ///
    /// Ascends to [`Parser::expr_or`]
    pub(super) fn expr_assign(&mut self) -> CompilerResult<Expr> {
        let mut expr = self.expr_or()?;

        loop {
            // check for assignment operator
            let token = self.current();

            let Ok(operator) = Operator::try_from(token.kind) else {
                break;
            };

            if !operator.is_assignment() {
                break;
            }

            // consume operator
            self.advance();

            let value = self.expr_assign()?;

            expr = Expr::spanned(
                expr.span.merged(&value.span),
                ExprKind::Assign(AssignExpr {
                    target: Box::new(expr),
                    operator: Spanned::spanned(token.span, operator),
                    value: Box::new(value),
                }),
            );
        }

        Ok(expr)
    }

    // MARK: Or
    /// Parses or expression
    ///
    /// Ascends to [`Parser::expr_and`]
    pub(super) fn expr_or(&mut self) -> CompilerResult<Expr> {
        let mut expr = self.expr_and()?;

        loop {
            let current = self.current();

            let operator = match &current.kind {
                TokenKind::PipePipe => {
                    self.advance(); // consume operator
                    current.kind
                }
                _ => break,
            };

            // we unwrap here because we just matched the token kind, aka it should be valid
            let operator = Operator::try_from(operator).unwrap();

            let right = self.expr_and()?;

            expr = Expr::spanned(
                expr.span.merged(&right.span),
                ExprKind::Binary(BinaryExpr {
                    left: Box::new(expr),
                    operator: Spanned::spanned(current.span, operator),
                    right: Box::new(right),
                }),
            );
        }

        Ok(expr)
    }

    // MARK: And
    /// Parses and expression
    ///
    /// Ascends to [`Parser::expr_equality`]
    pub(super) fn expr_and(&mut self) -> CompilerResult<Expr> {
        let mut expr = self.expr_equality()?;

        loop {
            let current = self.current();

            let operator = match &current.kind {
                TokenKind::AmpersandAmpersand => {
                    self.advance(); // consume operator
                    current.kind
                }
                _ => break,
            };

            // we unwrap here because we just matched the token kind, aka it should be valid
            let operator = Operator::try_from(operator).unwrap();

            let right = self.expr_equality()?;

            expr = Expr::spanned(
                expr.span.merged(&right.span),
                ExprKind::Binary(BinaryExpr {
                    left: Box::new(expr),
                    operator: Spanned::spanned(current.span, operator),
                    right: Box::new(right),
                }),
            );
        }

        Ok(expr)
    }

    // MARK: Equality
    /// Parses equality expressions (==, !=)
    ///
    /// Ascends to [`Parser::expr_comparison`]
    pub(super) fn expr_equality(&mut self) -> CompilerResult<Expr> {
        let mut expr = self.expr_comparison()?;

        loop {
            let current = self.current();

            let operator = match &current.kind {
                TokenKind::EqualEqual | TokenKind::BangEqual => {
                    self.advance(); // consume operator
                    current.kind
                }
                _ => break,
            };

            // we unwrap here because we just matched the token kind, aka it should be valid
            let operator = Operator::try_from(operator).unwrap();

            let right = self.expr_comparison()?;

            expr = Expr::spanned(
                expr.span.merged(&right.span),
                ExprKind::Binary(BinaryExpr {
                    left: Box::new(expr),
                    operator: Spanned::spanned(current.span, operator),
                    right: Box::new(right),
                }),
            );
        }

        Ok(expr)
    }

    // MARK: Comparison
    /// Parses comparison expressions (<, <=, >, >=)
    ///
    /// Ascends to [`Parser::expr_term`]
    pub(super) fn expr_comparison(&mut self) -> CompilerResult<Expr> {
        let mut expr = self.expr_term()?;

        loop {
            let current = self.current();

            let operator = match &current.kind {
                TokenKind::Less
                | TokenKind::LessEqual
                | TokenKind::Greater
                | TokenKind::GreaterEqual => {
                    self.advance(); // consume operator
                    current.kind
                }
                _ => break,
            };

            // we unwrap here because we just matched the token kind, aka it should be valid
            let operator = Operator::try_from(operator).unwrap();

            let right = self.expr_term()?;

            expr = Expr::spanned(
                expr.span.merged(&right.span),
                ExprKind::Binary(BinaryExpr {
                    left: Box::new(expr),
                    operator: Spanned::spanned(current.span, operator),
                    right: Box::new(right),
                }),
            );
        }

        Ok(expr)
    }

    // MARK: Term
    /// Parses term expressions (+, -)
    ///
    /// Ascends to [`Parser::expr_factor`]
    pub(super) fn expr_term(&mut self) -> CompilerResult<Expr> {
        let mut expr = self.expr_factor()?;

        loop {
            let current = self.current();

            let operator = match &current.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    self.advance(); // consume operator
                    current.kind
                }
                _ => break,
            };

            // we unwrap here because we just matched the token kind, aka it should be valid
            let operator = Operator::try_from(operator).unwrap();

            let right = self.expr_factor()?;

            expr = Expr::spanned(
                expr.span.merged(&right.span),
                ExprKind::Binary(BinaryExpr {
                    left: Box::new(expr),
                    operator: Spanned::spanned(current.span, operator),
                    right: Box::new(right),
                }),
            );
        }

        Ok(expr)
    }

    // MARK: Factor
    /// Parses factor expressions (*, /, %)
    ///
    /// Ascends to [`Parser::expr_unary`]
    pub(super) fn expr_factor(&mut self) -> CompilerResult<Expr> {
        let mut expr = self.expr_unary()?;

        loop {
            let current = self.current();

            let operator = match &current.kind {
                TokenKind::Asterisk | TokenKind::Slash | TokenKind::Percent => {
                    self.advance(); // consume operator
                    current.kind
                }
                _ => break,
            };

            // we unwrap here because we just matched the token kind, aka it should be valid
            let operator = Operator::try_from(operator).unwrap();

            let right = self.expr_unary()?;

            expr = Expr::spanned(
                expr.span.merged(&right.span),
                ExprKind::Binary(BinaryExpr {
                    left: Box::new(expr),
                    operator: Spanned::spanned(current.span, operator),
                    right: Box::new(right),
                }),
            );
        }

        Ok(expr)
    }

    // MARK: Unary
    /// Parses unary expressions (!, -)
    ///
    /// Ascends to [`Parser::expr_postfix`]
    pub(super) fn expr_unary(&mut self) -> CompilerResult<Expr> {
        let current = self.current();

        match &current.kind {
            TokenKind::Bang | TokenKind::Minus => {
                self.advance();

                // we unwrap here because we just matched the token kind, aka it should be valid
                let operator = Operator::try_from(current.kind).unwrap();
                let value = self.expr_unary()?; // recurse into unary, not call

                Ok(Expr::spanned(
                    current.span.merged(&value.span),
                    ExprKind::Unary(UnaryExpr {
                        operator: Spanned::spanned(current.span, operator),
                        value: Box::new(value),
                    }),
                ))
            }

            TokenKind::Plus => {
                self.advance(); // unary plus = no-op
                self.expr_unary()
            }

            _ => self.expr_postfix(),
        }
    }

    // MARK: Postfix
    /// Parses postfix expressions such as property access and calls
    ///
    /// Ascends to [`Parser::expr_primary`]
    pub(super) fn expr_postfix(&mut self) -> CompilerResult<Expr> {
        let mut expr = self.expr_primary()?;

        loop {
            let current = self.current();

            expr = match &current.kind {
                TokenKind::LeftParen => self.expr_finish_call(expr)?,
                TokenKind::Dot => self.expr_get(expr)?,

                _ => break,
            };
        }

        Ok(expr)
    }

    // MARK: Call
    /// Parses call expressions
    ///
    /// Finishes off parsing call expressions from [`Parser::expr_postfix`]
    pub(super) fn expr_finish_call(&mut self, mut expr: Expr) -> CompilerResult<Expr> {
        loop {
            if self.consume(TokenKind::LeftParen).is_err() {
                break;
            }

            let mut arguments = Vec::new();

            // parse arguments
            while !self.check(TokenKind::RightParen) {
                arguments.push(self.parse_expression()?);

                // if theres a comma, consume it and continue
                if self.consume(TokenKind::Comma).is_err() {
                    break;
                }
            }

            let right_paren = self.consume(TokenKind::RightParen)?;

            expr = Expr::spanned(
                expr.span.merged(&right_paren.span),
                ExprKind::Call(CallExpr {
                    callee: Box::new(expr),
                    arguments,
                }),
            );
        }

        Ok(expr)
    }

    // MARK: Get
    /// Parses a get expression
    pub(super) fn expr_get(&mut self, object: Expr) -> CompilerResult<Expr> {
        let dot_token = self.consume(TokenKind::Dot)?;

        let property = self.consume(TokenKind::Ident)?;

        Ok(Expr::spanned(
            dot_token.span.merged(&property.span),
            ExprKind::Get(GetExpr {
                object: Box::new(object),
                property: property.as_symbol(),
            }),
        ))
    }

    // MARK: Primary
    /// Parses primary expressions
    ///
    /// This is the highest-precedence level, so it matches instead of ascends.
    pub(super) fn expr_primary(&mut self) -> CompilerResult<Expr> {
        let current = self.current();

        match &current.kind {
            TokenKind::CharLiteral
            | TokenKind::FloatLiteral
            | TokenKind::IntLiteral
            | TokenKind::BoolLiteral
            | TokenKind::StringLiteral => self.expr_literal(),
            TokenKind::LeftParen => self.expr_tuple_group(),
            TokenKind::LeftBrace => self.expr_block(),
            TokenKind::If => self.expr_if(),
            TokenKind::Ident => self.expr_ident(),

            _ => Err(LumaError::new(
                ParserErrorKind::UnexpectedToken {
                    found: current.kind.clone(),
                },
                current.span,
            )),
        }
    }

    // MARK: Tuple/Group
    /// Parses tuples and grouped expressions `(...)`
    pub(super) fn expr_tuple_group(&mut self) -> CompilerResult<Expr> {
        let left_paren = self.consume(TokenKind::LeftParen)?;

        if let Ok(token) = self.consume(TokenKind::RightParen) {
            // empty tuple
            return Ok(Expr::spanned(
                left_paren.span.merged(&token.span),
                ExprKind::TupleLiteral(TupleExpr {
                    elements: Vec::new(),
                }),
            ));
        }

        let mut elements = Vec::new();

        // push the first element (this'll be used for grouping if no comma follows)
        elements.push(self.parse_expression()?);

        // tuple
        if self.consume(TokenKind::Comma).is_ok() {
            while !self.check(TokenKind::RightParen) {
                elements.push(self.parse_expression()?);

                if self.consume(TokenKind::Comma).is_err() {
                    break;
                }
            }

            self.consume(TokenKind::RightParen)?;

            return Ok(Expr::spanned(
                left_paren.span.merged(&elements.last().unwrap().span),
                ExprKind::TupleLiteral(TupleExpr { elements }),
            ));
        }

        // grouping
        self.consume(TokenKind::RightParen)?;

        let expr = elements.remove(0);

        Ok(Expr::spanned(
            left_paren.span.merged(&expr.span),
            ExprKind::Group(Box::new(expr)), 
        ))
    }

    // MARK: Block
    /// Parses a block expression
    pub(super) fn expr_block(&mut self) -> CompilerResult<Expr> {
        let left_brace = self.consume(TokenKind::LeftBrace)?;
        let mut statements: Vec<Stmt> = Vec::new();

        while self.current().kind != TokenKind::RightBrace && !self.is_at_end() {
            let stmt = self.parse_statement(Some(false))?;

            // optional semicolon for last statement in block
            let needs_semi =
                !self.check_next(TokenKind::RightBrace) && !self.check_next(TokenKind::Semicolon);

            if let Err(err) = self.consume(TokenKind::Semicolon)
                && needs_semi
            {
                // only error if we actually need a semicolon
                return Err(err);
            }

            statements.push(stmt);
        }

        let right_brace = self.consume(TokenKind::RightBrace)?;

        Ok(Expr::spanned(
            left_brace.span.merged(&right_brace.span),
            ExprKind::Block(BlockExpr { statements }),
        ))
    }

    // MARK: If
    /// Parses an if expression
    pub(super) fn expr_if(&mut self) -> CompilerResult<Expr> {
        // consume main branch
        let if_token = self.consume(TokenKind::If)?;

        let condition = self.parse_expression()?;

        let then_branch = self.expr_block()?;

        // check for else branch
        let else_branch = if self.consume(TokenKind::Else).is_ok() {
            if self.check(TokenKind::If) {
                Some(Box::new(self.expr_if()?))
            } else {
                Some(Box::new(self.expr_block()?))
            }
        } else {
            None
        };

        let mut span = if_token.span.merged(&then_branch.span);

        if let Some(else_branch) = &else_branch {
            span.merge(&else_branch.span);
        }

        Ok(Expr::spanned(
            span,
            ExprKind::If(IfExpr {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch,
            }),
        ))
    }

    // MARK: Literal
    /// Parses literal expressions
    pub(super) fn expr_literal(&mut self) -> CompilerResult<Expr> {
        let current = self.current();

        let kind = match &current.kind {
            TokenKind::CharLiteral => ExprKind::Literal(LiteralExpr::Char(
                current.lexeme.clone().chars().next().ok_or_else(|| {
                    LumaError::new(
                        ParserErrorKind::InvalidCharLiteral {
                            lexeme: current.lexeme.clone(),
                        },
                        current.span,
                    )
                })?,
            )),

            TokenKind::FloatLiteral => {
                let value = current.lexeme.parse::<f64>().map_err(|err| {
                    LumaError::new(
                        ParserErrorKind::InvalidFloatLiteral {
                            lexeme: current.lexeme.clone(),
                            source: err,
                        },
                        current.span,
                    )
                })?;

                ExprKind::Literal(LiteralExpr::Float(value))
            }

            TokenKind::IntLiteral => {
                let value = current.lexeme.parse::<u64>().map_err(|err| {
                    LumaError::new(
                        ParserErrorKind::InvalidIntegerLiteral {
                            lexeme: current.lexeme.clone(),
                            source: err,
                        },
                        current.span,
                    )
                })?;

                ExprKind::Literal(LiteralExpr::Int(value))
            }

            TokenKind::BoolLiteral => {
                let value = match current.lexeme.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => {
                        return Err(LumaError::new(
                            ParserErrorKind::InvalidBooleanLiteral {
                                lexeme: current.lexeme.clone(),
                            },
                            current.span,
                        ));
                    }
                };

                ExprKind::Literal(LiteralExpr::Bool(value))
            }

            TokenKind::StringLiteral => {
                ExprKind::Literal(LiteralExpr::String(current.lexeme.clone()))
            }

            _ => unreachable!("expr_literal called on non-literal token {:#?}", current),
        };

        // consume the token we just processed
        self.advance();

        Ok(Expr::spanned(current.span, kind))
    }

    // MARK: Ident
    /// Parses identifier expressions
    pub(super) fn expr_ident(&mut self) -> CompilerResult<Expr> {
        let ident = self.consume(TokenKind::Ident)?;

        if self.check(TokenKind::LeftBrace) {
            return self.expr_finish_struct(ident);
        }

        Ok(Expr::spanned(
            ident.span,
            ExprKind::Ident(IdentExpr {
                symbol: SymbolKind::named(ident.lexeme),
            }),
        ))
    }

    // MARK: Struct
    /// Parses struct literal expressions
    pub(super) fn expr_finish_struct(&mut self, ident: Token) -> CompilerResult<Expr> {
        // begin parsing fields
        let mut fields = Vec::new();

        self.consume(TokenKind::LeftBrace)?;

        // parse fields
        while !self.check(TokenKind::RightBrace) {
            // field ident
            let field_name_token = self.consume(TokenKind::Ident)?;

            // field value
            self.consume(TokenKind::Colon)?;

            let field_value = self.parse_expression()?;

            fields.push(StructFieldExpr {
                symbol: field_name_token.as_symbol(),
                value: field_value,
            });

            // this supports trailing commas
            if self.consume(TokenKind::Comma).is_err() {
                break;
            }
        }

        let right_brace = self.consume(TokenKind::RightBrace)?;

        Ok(Expr::spanned(
            ident.span.merged(&right_brace.span),
            ExprKind::Struct(StructExpr {
                symbol: ident.as_symbol(),
                fields,
            }),
        ))
    }
}
