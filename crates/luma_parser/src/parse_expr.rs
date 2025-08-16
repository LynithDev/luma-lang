use luma_core::{NumberRadix, ast::{BinaryOperator, ComparisonOperator, Expression, ExpressionKind, LogicalOperator, UnaryOperator}};
use luma_diagnostic::{LumaResult, ReporterExt};
use luma_lexer::tokens::{OperatorKind, PunctuationKind, TokenKind};

use crate::{diagnostics::ParserDiagnostic, LumaParser};

impl LumaParser<'_> {

    pub fn parse_expression(&mut self) -> LumaResult<Expression> {
        self.expr_scope()
    }

    // MARK: Scope
    fn expr_scope(&mut self) -> LumaResult<Expression> {
        if self.check(TokenKind::Punctuation(PunctuationKind::LeftBrace)) {
            let scope = self.consume_scope()?;

            return Ok(Expression {
                cursor: scope.cursor,
                span: scope.span,
                kind: ExpressionKind::Scope(scope.statements),
            });
        }

        self.expr_assignment()
    }

    // MARK: Assignment
    fn expr_assignment(&mut self) -> LumaResult<Expression> {
        let mut left = self.expr_or()?;

        while let TokenKind::Operator(op_kind) = self.current().kind && op_kind.is_assign_op() {
            let (span, cursor) = self.advance().pos();
            let right = self.expr_assignment()?;

            match left.kind {
                ExpressionKind::Variable(name) => {
                    left = Expression {
                        cursor,
                        span: left.span.merge(&right.span),
                        kind: ExpressionKind::Assign(
                            name,
                            op_kind.as_assign_operator().unwrap(),
                            Box::new(right)
                        ),
                    }
                },
                ExpressionKind::ArrayGet(array_expr, index_expr ) => {
                    left = Expression {
                        cursor,
                        span: left.span.merge(&right.span),
                        kind: ExpressionKind::ArraySet(
                            array_expr,
                            index_expr,
                            Box::new(right)
                        ),
                    }
                }
                _ => return Err(self.diagnostic_at(ParserDiagnostic::InvalidLeftHandSide(Box::new(left.kind)), span, cursor))
            }
        }

        Ok(left)
    }

    // MARK: Or
    fn expr_or(&mut self) -> LumaResult<Expression> {
        let mut left = self.expr_and()?;

        while self.check(TokenKind::Operator(OperatorKind::Or)) {
            let op_token = self.advance().to_owned();
            let right = self.expr_and()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Logical(
                    Box::new(left),
                    LogicalOperator::Or,
                    Box::new(right)
                ),
            };
        }

        Ok(left)
    }

    // MARK: And
    fn expr_and(&mut self) -> LumaResult<Expression> {
        let mut left = self.expr_bitwise_or()?;

        while self.check(TokenKind::Operator(OperatorKind::And)) {
            let op_token = self.advance().to_owned();
            let right = self.expr_bitwise_or()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Logical(
                    Box::new(left),
                    LogicalOperator::And,
                    Box::new(right)
                ),
            };
        }

        Ok(left)
    }

    // MARK: Bitwise OR
    fn expr_bitwise_or(&mut self) -> LumaResult<Expression> {
        let mut left = self.expr_bitwise_xor()?;

        while self.check(TokenKind::Operator(OperatorKind::BitwiseOr)) {
            let op_token = self.advance().to_owned();
            let right = self.expr_bitwise_xor()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Binary(
                    Box::new(left),
                    BinaryOperator::BitwiseOr,
                    Box::new(right)
                ),
            };
        }

        Ok(left)
    }

    // MARK: Bitwise XOR
    fn expr_bitwise_xor(&mut self) -> LumaResult<Expression> {
        let mut left = self.expr_bitwise_and()?;

        while self.check(TokenKind::Operator(OperatorKind::BitwiseXor)) {
            let op_token = self.advance().to_owned();
            let right = self.expr_bitwise_and()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Binary(
                    Box::new(left),
                    BinaryOperator::BitwiseXor,
                    Box::new(right)
                ),
            };
        }

        Ok(left)
    }

    // MARK: Bitwise AND
    fn expr_bitwise_and(&mut self) -> LumaResult<Expression> {
        let mut left = self.expr_cmp()?;

        while self.check(TokenKind::Operator(OperatorKind::BitwiseAnd)) {
            let op_token = self.advance().to_owned();
            let right = self.expr_cmp()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Binary(
                    Box::new(left),
                    BinaryOperator::BitwiseAnd,
                    Box::new(right)
                ),
            };
        }

        Ok(left)
    }

    // MARK: Comparison
    fn expr_cmp(&mut self) -> LumaResult<Expression> {
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
                kind: ExpressionKind::Comparison(
                    Box::new(left),
                    ComparisonOperator::try_from(*op_token.kind.as_operator().unwrap()).unwrap(),
                    Box::new(right)
                ),
            };
        }

        Ok(left)
    }

    // MARK: Bitwise shift
    fn expr_bitwise_shift(&mut self) -> LumaResult<Expression> {
        let mut left = self.expr_additive()?;

        while matches!(self.current().kind, TokenKind::Operator(OperatorKind::ShiftLeft) | TokenKind::Operator(OperatorKind::ShiftRight)) {
            let op_token = self.advance().to_owned();
            let right = self.expr_additive()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Binary(
                    Box::new(left),
                    BinaryOperator::try_from(*op_token.kind.as_operator().unwrap()).unwrap(),
                    Box::new(right)
                ),
            };
        }

        Ok(left)
    }

    // MARK: Binary + -
    fn expr_additive(&mut self) -> LumaResult<Expression> {
        let mut left = self.expr_multiplicative()?;

        while matches!(self.current().kind, TokenKind::Operator(OperatorKind::Plus) | TokenKind::Operator(OperatorKind::Minus)) {
            let op_token = self.advance().to_owned();
            let bin_op = BinaryOperator::try_from(*op_token.kind.as_operator().unwrap()).unwrap();

            let right = self.expr_multiplicative()?;

            left = Expression {
                cursor: op_token.cursor,
                span: left.span.merge(&right.span),
                kind: ExpressionKind::Binary(
                    Box::new(left),
                    bin_op,
                    Box::new(right)
                ),
            };
        }

        Ok(left)
    }

    // MARK: Binary * / %
    fn expr_multiplicative(&mut self) -> LumaResult<Expression> {
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
                kind: ExpressionKind::Binary(
                    Box::new(left),
                    bin_op,
                    Box::new(right)
                ),
            };
        }

        Ok(left)
    }

    // MARK: Unary
    fn expr_unary(&mut self) -> LumaResult<Expression> {
        if matches!(
            self.current().kind, 
            TokenKind::Operator(OperatorKind::Plus) 
            | TokenKind::Operator(OperatorKind::Minus) 
            | TokenKind::Operator(OperatorKind::Not)
            | TokenKind::Operator(OperatorKind::BitwiseNot)
        ) {
            let op_token = self.advance().to_owned();
            let right = self.expr_unary()?;

            return Ok(Expression {
                cursor: op_token.cursor,
                span: op_token.span.merge(&right.span),
                kind: ExpressionKind::Unary(
                    UnaryOperator::try_from(*op_token.kind.as_operator().unwrap()).unwrap(),
                    Box::new(right)
                ),
            });
        }

        self.expr_call()
    }

    // MARK: Call
    fn expr_call(&mut self) -> LumaResult<Expression> {
        let mut expr = self.expr_primary()?;

        loop {
            match self.current().kind {
                TokenKind::Punctuation(PunctuationKind::LeftParen) => {
                    expr = self.parse_invoke(expr)?;
                }
                TokenKind::Punctuation(PunctuationKind::Dot) => {
                    expr = self.parse_get(expr)?;
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    // MARK: Primary
    fn expr_primary(&mut self) -> LumaResult<Expression> {
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
                    kind: ExpressionKind::Group(Box::new(expr)),
                }
            },

            // Identifier parsing
            TokenKind::Identifier => {
                Expression {
                    kind: ExpressionKind::Variable(current.lexeme.clone()),
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
    fn parse_get(&mut self, target: Expression) -> LumaResult<Expression> {
        let dot_token = self.consume(TokenKind::Punctuation(PunctuationKind::Dot))?;
        let (span, cursor) = dot_token.pos();
        let identifier = self.consume(TokenKind::Identifier)?;
        let name = identifier.lexeme.clone();

        Ok(Expression {
            cursor,
            span: span.merge(&identifier.span),
            kind: ExpressionKind::Get {
                object: Box::new(target),
                property: name,
            },
        })
    }

    // MARK: Invoke
    fn parse_invoke(&mut self, callee: Expression) -> LumaResult<Expression> {
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
