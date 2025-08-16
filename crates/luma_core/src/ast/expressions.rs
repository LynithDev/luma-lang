use crate::{Cursor, Span};

use crate::{ast::{BinaryOperator, ComparisonOperator, ConditionalBranch, LogicalOperator, Operator, Statement, UnaryOperator}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
    pub cursor: Cursor,
}

#[derive(crate::Display, Debug, Clone, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum ExpressionKind {
    If {
        main_expr: Box<ConditionalBranch>,
        branches: Option<Vec<ConditionalBranch>>,
        else_expr: Box<Expression>
    },
    Invoke {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Assign(String, Operator, Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Comparison(Box<Expression>, ComparisonOperator, Box<Expression>),
    Logical(Box<Expression>, LogicalOperator, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    Group(Box<Expression>),
    Variable(String),
    Scope(Vec<Statement>),
    Literal {
        kind: LiteralKind,
        value: String,
    },
    Get {
        object: Box<Expression>,
        property: String,
    },
    ArrayGet(Box<Expression>, Box<Expression>),
    ArraySet(Box<Expression>, Box<Expression>, Box<Expression>),
}

#[derive(crate::Display, Debug, Clone, Copy, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum LiteralKind {
    Integer,
    Decimal,
    String,
    Boolean,
}
