use luma_core::{ast::{prelude::*, AstSymbol}, NumberRadix};
use luma_diagnostic::{DiagnosticResult, ReporterExt};
use luma_lexer::tokens::{OperatorKind, PunctuationKind, TokenKind};

use crate::{diagnostics::ParserDiagnostic, LumaParser};

impl LumaParser<'_> {

    pub fn parse_expression(&mut self) -> DiagnosticResult<Expression> {
        self.expr_scope()
    }

    // MARK: Scope
    fn expr_scope(&mut self) -> DiagnosticResult<Expression> {
        if self.check(TokenKind::Punctuation(PunctuationKind::LeftBrace)) {
            return self.consume_scope();
        }

        self.expr_assignment()
    }

    // MARK: Assignment
    fn expr_assignment(&mut self) -> DiagnosticResult<Expression> {
        let mut left = self.expr_or()?;

        while let TokenKind::Operator(op_kind) = self.current().kind && op_kind.is_assign_op() {
            let (span, cursor) = self.advance().pos();
            let right = self.expr_assignment()?;

            match left.kind {
                ExpressionKind::Variable { symbol } => {
                    left = Expression {
                        cursor,
                        span: left.span.merge(&right.span),
                        kind: ExpressionKind::Assign {
                            symbol,
                            operator: op_kind.as_assign_operator().unwrap(),
                            value: Box::new(right)
                        },
                    }
                },
                ExpressionKind::ArrayGet { array, index } => {
                    left = Expression {
                        cursor,
                        span: left.span.merge(&right.span),
                        kind: ExpressionKind::ArraySet {
                            array,
                            index,
                            value: Box::new(right)
                        },
                    }
                }
                _ => return Err(self.diagnostic_at(ParserDiagnostic::InvalidLeftHandSide(Box::new(left.kind)), span, cursor))
            }
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
                    operator: ComparisonOperator::try_from(*op_token.kind.as_operator().unwrap()).unwrap(),
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
