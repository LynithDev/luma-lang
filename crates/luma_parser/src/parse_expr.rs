use luma_core::{ast::{prelude::*, AstSymbol}, NumberRadix};
use luma_diagnostic::{DiagnosticResult, ReporterExt};
use luma_lexer::tokens::{KeywordKind, OperatorKind, PunctuationKind, TokenKind};

use crate::{diagnostics::ParserDiagnostic, LumaParser};

impl LumaParser<'_> {

    pub fn parse_expression(&mut self) -> DiagnosticResult<Expression> {
        self.expr_if()
    }

    // MARK: If
    fn expr_if(&mut self) -> DiagnosticResult<Expression> {
        if !self.check(TokenKind::Keyword(KeywordKind::If)) {
            return self.expr_scope();
        }

        let (mut span, cursor) = self.advance().pos();

        let main_branch = self.parse_conditional_branch()?;
        span = span.merge(&main_branch.body.span);

        // check for `else if`
        let mut branches: Vec<ConditionalBranch> = Vec::new();
        let mut else_branch: Option<Box<Expression>> = None;

        while self.consume(TokenKind::Keyword(KeywordKind::Else)).is_ok() {
            if self.consume(TokenKind::Keyword(KeywordKind::If)).is_ok() {
                let branch = self.parse_conditional_branch()?;
                span = span.merge(&branch.body.span);

                branches.push(branch);
            } else {
                let body = self.parse_expression()?;

                else_branch = Some(Box::new(body));
                break;
            }
        }

        Ok(Expression {
            kind: ExpressionKind::If {
                main_branch: Box::new(main_branch),
                branches,
                else_branch,
            },
            span,
            cursor,
        })
    }

    // MARK: Scope
    fn expr_scope(&mut self) -> DiagnosticResult<Expression> {
        if !self.check(TokenKind::Punctuation(PunctuationKind::LeftBrace)) {
            return self.expr_assignment();
        }

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

                    let is_control_flow = matches!(
                        kind,
                        StatementKind::Break { .. }
                            | StatementKind::Continue { .. }
                            | StatementKind::Return { .. }
                    );

                    if is_control_flow || is_implicit_return {
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

    // MARK: Assignment
    fn expr_assignment(&mut self) -> DiagnosticResult<Expression> {
        let mut left = self.expr_or()?;

        while let TokenKind::Operator(op_kind) = self.current().kind && op_kind.is_assign_op() {
            let (span, cursor) = self.advance().pos();
            let right = if op_kind == OperatorKind::Equals {
                self.expr_assignment()?
            } else {
                let assign_operator = op_kind.as_assign_operator().unwrap();
                let value = self.expr_assignment()?;

                Expression {
                    cursor,
                    span: left.span.merge(&value.span),
                    kind: match assign_operator {
                        Operator::Binary(bin_op) => ExpressionKind::Binary {
                            left: Box::new(left.clone()),
                            operator: bin_op,
                            right: Box::new(value),
                        },
                        Operator::Logical(log_op) => ExpressionKind::Logical {
                            left: Box::new(left.clone()),
                            operator: log_op,
                            right: Box::new(value),
                        },
                        _ => unreachable!("assign operator must be binary or logical"),
                    }
                }
            };

            left = Expression {
                cursor,
                span: span.merge(&left.span).merge(&right.span),
                kind: ExpressionKind::Assign {
                    target: Box::new(left),
                    value: Box::new(right)
                },
            };
        }

        Ok(left)
    }

    // MARK: Or
    fn expr_or(&mut self) -> DiagnosticResult<Expression> {
        let mut left = self.expr_and()?;

        while self.check(TokenKind::Operator(OperatorKind::Or)) {
            let op_token = self.advance().to_owned();
            let right = self.expr_and()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Logical {
                    left: Box::new(left),
                    operator: LogicalOperator::Or,
                    right: Box::new(right)
                },
            };
        }

        Ok(left)
    }

    // MARK: And
    fn expr_and(&mut self) -> DiagnosticResult<Expression> {
        let mut left = self.expr_bitwise_or()?;

        while self.check(TokenKind::Operator(OperatorKind::And)) {
            let op_token = self.advance().to_owned();
            let right = self.expr_bitwise_or()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Logical {
                    left: Box::new(left),
                    operator: LogicalOperator::And,
                    right: Box::new(right)
                },
            };
        }

        Ok(left)
    }

    // MARK: Bitwise OR
    fn expr_bitwise_or(&mut self) -> DiagnosticResult<Expression> {
        let mut left = self.expr_bitwise_xor()?;

        while self.check(TokenKind::Operator(OperatorKind::BitwiseOr)) {
            let op_token = self.advance().to_owned();
            let right = self.expr_bitwise_xor()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Binary {
                    left: Box::new(left),
                    operator: BinaryOperator::BitwiseOr,
                    right: Box::new(right)
                },
            };
        }

        Ok(left)
    }

    // MARK: Bitwise XOR
    fn expr_bitwise_xor(&mut self) -> DiagnosticResult<Expression> {
        let mut left = self.expr_bitwise_and()?;

        while self.check(TokenKind::Operator(OperatorKind::BitwiseXor)) {
            let op_token = self.advance().to_owned();
            let right = self.expr_bitwise_and()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Binary {
                    left: Box::new(left),
                    operator: BinaryOperator::BitwiseXor,
                    right: Box::new(right)
                },
            };
        }

        Ok(left)
    }

    // MARK: Bitwise AND
    fn expr_bitwise_and(&mut self) -> DiagnosticResult<Expression> {
        let mut left = self.expr_cmp()?;

        while self.check(TokenKind::Operator(OperatorKind::BitwiseAnd)) {
            let op_token = self.advance().to_owned();
            let right = self.expr_cmp()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Binary {
                    left: Box::new(left),
                    operator: BinaryOperator::BitwiseAnd,
                    right: Box::new(right)
                },
            };
        }

        Ok(left)
    }

    // MARK: Comparison
    fn expr_cmp(&mut self) -> DiagnosticResult<Expression> {
        let mut left = self.expr_bitwise_shift()?;

        while matches!(
            self.current().kind, 
            TokenKind::Operator(OperatorKind::EqualsEquals)
            | TokenKind::Operator(OperatorKind::NotEquals)
            | TokenKind::Operator(OperatorKind::LessThan)
            | TokenKind::Operator(OperatorKind::GreaterThan)
            | TokenKind::Operator(OperatorKind::LessThanOrEqual)
            | TokenKind::Operator(OperatorKind::GreaterThanOrEqual)
        ) {
            let op_token = self.advance().to_owned();
            let right = self.expr_bitwise_shift()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Comparison {
                    left: Box::new(left),
                    operator: ComparisonOperator::try_from(*op_token.kind.as_operator().unwrap()).unwrap_or_else(|_| panic!("invalid comparison operator {:?}", op_token.kind)),
                    right: Box::new(right)
                },
            };
        }

        Ok(left)
    }

    // MARK: Bitwise shift
    fn expr_bitwise_shift(&mut self) -> DiagnosticResult<Expression> {
        let mut left = self.expr_additive()?;

        while matches!(self.current().kind, TokenKind::Operator(OperatorKind::ShiftLeft) | TokenKind::Operator(OperatorKind::ShiftRight)) {
            let op_token = self.advance().to_owned();
            let right = self.expr_additive()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Binary {
                    left: Box::new(left),
                    operator: BinaryOperator::try_from(*op_token.kind.as_operator().unwrap()).unwrap(),
                    right: Box::new(right)
                },
            };
        }

        Ok(left)
    }

    // MARK: Binary + -
    fn expr_additive(&mut self) -> DiagnosticResult<Expression> {
        let mut left = self.expr_multiplicative()?;

        while matches!(self.current().kind, TokenKind::Operator(OperatorKind::Plus) | TokenKind::Operator(OperatorKind::Minus)) {
            let op_token = self.advance().to_owned();
            let bin_op = BinaryOperator::try_from(*op_token.kind.as_operator().unwrap()).unwrap();

            let right = self.expr_multiplicative()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Binary {
                    left: Box::new(left),
                    operator: bin_op,
                    right: Box::new(right)
                },
            };
        }

        Ok(left)
    }

    // MARK: Binary * / %
    fn expr_multiplicative(&mut self) -> DiagnosticResult<Expression> {
        let mut left = self.expr_unary()?;

        while matches!(
            self.current().kind, 
            TokenKind::Operator(OperatorKind::Asterisk) 
            | TokenKind::Operator(OperatorKind::Slash) 
            | TokenKind::Operator(OperatorKind::Percent)
        ) {
            let op_token = self.advance().to_owned();
            let bin_op = BinaryOperator::try_from(*op_token.kind.as_operator().unwrap()).unwrap();

            let right = self.expr_unary()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Binary {
                    left: Box::new(left),
                    operator: bin_op,
                    right: Box::new(right)
                },
            };
        }

        Ok(left)
    }

    // MARK: Unary
    fn expr_unary(&mut self) -> DiagnosticResult<Expression> {
        if matches!(
            self.current().kind, 
            TokenKind::Operator(OperatorKind::Plus) 
            | TokenKind::Operator(OperatorKind::Minus) 
            | TokenKind::Operator(OperatorKind::Not)
            | TokenKind::Operator(OperatorKind::BitwiseNot)
        ) {
            let op_token = self.advance().to_owned();
            let right = self.expr_unary()?;

            if op_token.kind == TokenKind::Operator(OperatorKind::Plus) {
                return Ok(right);
            }

            return Ok(Expression {
                cursor: op_token.cursor,
                span: op_token.span.merge(&right.span),
                kind: ExpressionKind::Unary {
                    operator: UnaryOperator::try_from(*op_token.kind.as_operator().unwrap()).unwrap(),
                    value: Box::new(right)
                },
            });
        }

        self.expr_call()
    }

    // MARK: Call
    fn expr_call(&mut self) -> DiagnosticResult<Expression> {
        let mut expr = self.expr_primary()?;

        loop {
            match self.current().kind {
                TokenKind::Punctuation(PunctuationKind::LeftParen) => {
                    expr = self.parse_invoke(expr)?;
                }
                TokenKind::Punctuation(PunctuationKind::Dot) => {
                    expr = self.parse_get(expr)?;
                }
                TokenKind::Punctuation(PunctuationKind::LeftBracket) => {
                    expr = self.parse_array_get(expr)?;
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    // MARK: Primary
    fn expr_primary(&mut self) -> DiagnosticResult<Expression> {
        let current = self.current();

        let expr = match current.kind {
            // Grouping
            TokenKind::Punctuation(PunctuationKind::LeftParen) => {
                let (span, cursor) = current.pos();

                self.advance(); // skip '('
                let expr = self.parse_expression()?;
                self.expect(TokenKind::Punctuation(PunctuationKind::RightParen))?;

                Expression {
                    cursor,
                    span: span.merge(&expr.span),
                    kind: ExpressionKind::Group {
                        inner: Box::new(expr),
                    },
                }
            },

            // Identifier parsing
            TokenKind::Identifier => {
                Expression {
                    kind: ExpressionKind::Variable {
                        symbol: AstSymbol::new(
                            current.lexeme.clone(),
                            current.span,
                            current.cursor,
                        )
                    },
                    cursor: current.cursor,
                    span: current.span,
                }
            },

            TokenKind::Literal(kind) => {
                let mut value: String = current.lexeme.clone();

                if kind == luma_lexer::tokens::LiteralKind::Integer && value.starts_with('0') && let Some(char) = value.chars().nth(1) && !char.is_ascii_digit() {
                    // we try to get the number radix
                    let radix = NumberRadix::try_from(char)
                        .map_err(|_| self.diagnostic(ParserDiagnostic::InvalidNumberLiteral(value.clone())))?;

                    // parse the lexeme value with the radix
                    let parsed = u64::from_str_radix(&value[2..], radix as u32)
                        .map_err(|_| self.diagnostic(ParserDiagnostic::InvalidNumberLiteral(value)))?;

                    value = parsed.to_string()
                };

                Expression {
                    cursor: current.cursor,
                    span: current.span,
                    kind: ExpressionKind::Literal { 
                        kind: kind.into(), 
                        value
                    },
                }
            },

            TokenKind::Punctuation(PunctuationKind::LeftBracket) => {
                let (span, cursor) = current.pos();
                self.advance(); // consume '['

                let mut elements: Vec<Expression> = Vec::new();
                let mut ty = TypeKind::Unknown;
                let mut size: Option<Box<Expression>> = None;

                if !self.check(TokenKind::Punctuation(PunctuationKind::RightBracket)) {
                    loop {
                        let element = self.parse_expression()?;
                        elements.push(element);

                        if self.check(TokenKind::Punctuation(PunctuationKind::RightBracket)) {
                            break;
                        } else if self.check(TokenKind::Punctuation(PunctuationKind::Semicolon)) {
                            // array with type and size
                            self.advance(); // consume ';'

                            // parse type
                            let type_expr = elements.pop().ok_or_else(|| self.diagnostic_at(
                                ParserDiagnostic::MissingArrayTypeAnnotation,
                                span,
                                cursor,
                            ))?;

                            let type_identifier = if let ExpressionKind::Variable { symbol } = type_expr.kind {
                                symbol
                            } else {
                                return Err(self.diagnostic_at(
                                    ParserDiagnostic::MissingArrayTypeAnnotation,
                                    type_expr.span,
                                    type_expr.cursor,
                                ));
                            };
                            
                            ty = TypeKind::from(type_identifier.name.as_str());

                            // parse size
                            size = Some(Box::new(self.parse_expression()?));

                            break;
                        } else {
                            self.consume(TokenKind::Punctuation(PunctuationKind::Comma))?;
                        }
                    }
                }

                let rbracket = self.expect(TokenKind::Punctuation(PunctuationKind::RightBracket))?;
                let (rbracket_span, _) = rbracket.pos();

                Expression {
                    cursor,
                    span: span.merge(&rbracket_span),
                    kind: ExpressionKind::ArrayLiteral {
                        elements,
                        inner_type: ty,
                        size,
                    },
                }
            }

            _ => {
                let previous = self.advance().to_owned();

                return Err(self.diagnostic_at(
                    ParserDiagnostic::UnexpectedToken(previous.kind),
                    previous.span,
                    previous.cursor,
                ))
            },
        };

        self.advance(); // consume primary token

        Ok(expr)
    }

    // MARK: Get
    fn parse_get(&mut self, target: Expression) -> DiagnosticResult<Expression> {
        let dot_token = self.consume(TokenKind::Punctuation(PunctuationKind::Dot))?;
        let (span, cursor) = dot_token.pos();
        let identifier = self.consume(TokenKind::Identifier)?;

        Ok(Expression {
            cursor,
            span: span.merge(&identifier.span),
            kind: ExpressionKind::Get {
                object: Box::new(target),
                property_symbol: AstSymbol::new(identifier.lexeme.clone(), identifier.span, identifier.cursor),
            },
        })
    }

    // MARK: Array Get
    fn parse_array_get(&mut self, target: Expression) -> DiagnosticResult<Expression> {
        let lbrace = self.consume(TokenKind::Punctuation(PunctuationKind::LeftBracket))?;
        let (span, cursor) = lbrace.pos();

        let index = self.parse_expression()?;
        let rbrace = self.consume(TokenKind::Punctuation(PunctuationKind::RightBracket))?;
        let (rbrace_span, _) = rbrace.pos();

        Ok(Expression {
            cursor,
            span: span.merge(&rbrace_span),
            kind: ExpressionKind::ArrayGet {
                array: Box::new(target),
                index: Box::new(index),
            },
        })
    }

    // MARK: Invoke
    fn parse_invoke(&mut self, callee: Expression) -> DiagnosticResult<Expression> {
        let (span, cursor) = self.consume(TokenKind::Punctuation(PunctuationKind::LeftParen))?.pos();
        
        let mut arguments = Vec::new();
        
        if !self.check(TokenKind::Punctuation(PunctuationKind::RightParen)) {
            loop {
                arguments.push(self.parse_expression()?);

                if self.check(TokenKind::Punctuation(PunctuationKind::RightParen)) {
                    break;
                }

                self.consume(TokenKind::Punctuation(PunctuationKind::Comma))?;
            }
        }

        let (rparen_span, _) = self.consume(TokenKind::Punctuation(PunctuationKind::RightParen))?.pos();

        Ok(Expression {
            cursor,
            span: span.merge(&rparen_span),
            kind: ExpressionKind::Invoke {
                callee: Box::new(callee),
                arguments,
            },
        })
    }

}
