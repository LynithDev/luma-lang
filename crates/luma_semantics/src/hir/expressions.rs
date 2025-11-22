use luma_core::SymbolId;
use luma_core::types::TypeKind;
use luma_core::{Cursor, Display, Span};

use luma_core::{operators::*};

use crate::hir::statements::HirStatement;
use crate::hir::HirConditionalBranch;

#[derive(Debug, Clone, PartialEq)]
pub struct HirExpression {
    pub kind: HirExpressionKind,
    pub ty: TypeKind,
    pub span: Span,
    pub cursor: Cursor,
}

#[derive(Display, Debug, Clone, PartialEq)]
#[display(case = "snake_case")]
pub enum HirExpressionKind {
    If {
        main_expr: Box<HirConditionalBranch>,
        branches: Vec<HirConditionalBranch>,
        else_expr: Option<Box<HirExpression>>
    },
    Invoke {
        callee: Box<HirExpression>,
        arguments: Vec<HirExpression>,
    },
    Assign {
        symbol_id: SymbolId,
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
        statements: Vec<HirStatement>,
        value: Option<Box<HirExpression>>,
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

impl From<&HirLiteralKind> for TypeKind {
    fn from(value: &HirLiteralKind) -> Self {
        match value {
            HirLiteralKind::String(_) => TypeKind::String,
            HirLiteralKind::Boolean(_) => TypeKind::Boolean,
            HirLiteralKind::Integer(HirLiteralIntegerKind::Int8(_)) => TypeKind::Int8,
            HirLiteralKind::Integer(HirLiteralIntegerKind::Int16(_)) => TypeKind::Int16,
            HirLiteralKind::Integer(HirLiteralIntegerKind::Int32(_)) => TypeKind::Int32,
            HirLiteralKind::Integer(HirLiteralIntegerKind::Int64(_)) => TypeKind::Int64,
            HirLiteralKind::Integer(HirLiteralIntegerKind::UInt8(_)) => TypeKind::UInt8,
            HirLiteralKind::Integer(HirLiteralIntegerKind::UInt16(_)) => TypeKind::UInt16,
            HirLiteralKind::Integer(HirLiteralIntegerKind::UInt32(_)) => TypeKind::UInt32,
            HirLiteralKind::Integer(HirLiteralIntegerKind::UInt64(_)) => TypeKind::UInt64,
            HirLiteralKind::Float(HirLiteralFloatKind::Float32(_)) => TypeKind::Float32,
            HirLiteralKind::Float(HirLiteralFloatKind::Float64(_)) => TypeKind::Float64,
        }
    }
}
