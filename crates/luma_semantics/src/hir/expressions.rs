use luma_core::SymbolId;
use luma_core::{Cursor, Display, Span};

use luma_core::{operators::*};

use crate::hir::statements::HirStatement;
use crate::hir::HirConditionalBranch;

#[derive(Debug, Clone, PartialEq)]
pub struct HirExpression {
    pub kind: HirExpressionKind,
    pub span: Span,
    pub cursor: Cursor,
}

#[derive(Display, Debug, Clone, PartialEq)]
#[display(case = "snake_case")]
pub enum HirExpressionKind {
    If {
        main_expr: Box<HirConditionalBranch>,
        branches: Option<Vec<HirConditionalBranch>>,
        else_expr: Box<HirExpression>
    },
    Invoke {
        callee: Box<HirExpression>,
        arguments: Vec<HirExpression>,
    },
    Assign {
        symbol_id: SymbolId,
        operator: Operator, 
        value: Box<HirExpression>
    },
    Binary {
        left: Box<HirExpression>, 
        operator: BinaryOperator, 
        right: Box<HirExpression>
    },
    Comparison {
        left: Box<HirExpression>, 
        operator: ComparisonOperator, 
        right: Box<HirExpression>
    },
    Logical {
        left: Box<HirExpression>, 
        operator: LogicalOperator, 
        right: Box<HirExpression>
    },
    Unary {
        operator: UnaryOperator, 
        value: Box<HirExpression>
    },
    Group {
        inner: Box<HirExpression>
    },
    Variable {
        symbol_id: SymbolId,
    },
    Scope {
        statements: Vec<HirStatement>
    },
    Literal {
        kind: HirLiteralKind,
    },
    Get {
        object: Box<HirExpression>,
        property_symbol_id: SymbolId,
    },
    ArrayGet {
        array: Box<HirExpression>,
        index: Box<HirExpression>,
    },
    ArraySet {
        array: Box<HirExpression>,
        index: Box<HirExpression>,
        value: Box<HirExpression>,
    },
}

#[derive(Display, Debug, Clone, PartialEq)]
#[display(case = "snake_case")]
pub enum HirLiteralKind {
    Integer(HirLiteralIntegerKind),
    Float(HirLiteralFloatKind),
    String(String),
    Boolean(bool),
}

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum HirLiteralIntegerKind {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
}

#[derive(Display, Debug, Clone, Copy, PartialEq)]
#[display(case = "snake_case")]
pub enum HirLiteralFloatKind {
    Float32(f32),
    Float64(f64),
}