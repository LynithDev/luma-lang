use crate::ast::AstSymbol;
use crate::{Cursor, Span};

use crate::{
    ast::{ConditionalBranch, statements::Statement},
    operators::*,
};

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
        main_branch: Box<ConditionalBranch>,
        branches: Vec<ConditionalBranch>,
        else_branch: Option<Box<Expression>>,
    },
    Invoke {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Assign {
        symbol: AstSymbol,
        value: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Comparison {
        left: Box<Expression>,
        operator: ComparisonOperator,
        right: Box<Expression>,
    },
    Logical {
        left: Box<Expression>,
        operator: LogicalOperator,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        value: Box<Expression>,
    },
    Group {
        inner: Box<Expression>,
    },
    Variable {
        symbol: AstSymbol,
    },
    Scope {
        statements: Vec<Statement>,
        /// The implicit value of the scope (last expression with no semicolon)
        block_value: Option<Box<Expression>>,
    },
    Literal {
        kind: LiteralKind,
        value: String,
    },
    Get {
        object: Box<Expression>,
        property_symbol: AstSymbol,
    },
    ArrayGet {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    ArraySet {
        array: Box<Expression>,
        index: Box<Expression>,
        value: Box<Expression>,
    },
}

#[derive(crate::Display, Debug, Clone, Copy, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum LiteralKind {
    Integer,
    Float,
    String,
    Boolean,
}
