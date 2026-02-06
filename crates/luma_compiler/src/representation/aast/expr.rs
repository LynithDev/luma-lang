use luma_core::Span;
use strum::Display;

use crate::{Operator, TypeKind, aast::*, stages::analyzer::scopes::ScopeId};

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotExpr {
    pub item: AnnotExprKind,
    pub ty: TypeKind,
    pub scope_id: ScopeId,
    pub span: Span,
}

impl AnnotExpr {
    pub fn new(span: Span, item: AnnotExprKind, ty: TypeKind, scope_id: ScopeId) -> Self {
        Self {
            item,
            ty,
            scope_id,
            span,
        }
    }
}

#[derive(Display, Debug, Clone, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum AnnotExprKind {
    Assign(AssignAnnotExpr),
    Binary(BinaryAnnotExpr),
    Block(BlockAnnotExpr),
    Call(CallAnnotExpr),
    Get(GetAnnotExpr),
    Group(Box<AnnotExpr>),
    Ident(IdentAnnotExpr),
    If(IfAnnotExpr),
    Literal(LiteralAnnotExpr),
    Struct(StructAnnotExpr),
    TupleLiteral(TupleAnnotExpr),
    Unary(UnaryAnnotExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignAnnotExpr {
    pub target: Box<AnnotExpr>,
    pub operator: Operator,
    pub value: Box<AnnotExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryAnnotExpr {
    pub left: Box<AnnotExpr>,
    pub operator: Operator,
    pub right: Box<AnnotExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockAnnotExpr {
    pub statements: Vec<AnnotStmt>,
}

impl BlockAnnotExpr {
    pub fn return_value(&self) -> Option<&AnnotExpr> {
        let stmt = self.statements.last()?;

        match &stmt.item {
            AnnotStmtKind::Expr(expr) => Some(expr),
            AnnotStmtKind::Return(ret) => ret
                .value
                .as_ref(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallAnnotExpr {
    pub callee: Box<AnnotExpr>,
    pub arguments: Vec<AnnotExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetAnnotExpr {
    pub object: Box<AnnotExpr>,
    pub property: AnnotSymbol,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentAnnotExpr {
    pub symbol: AnnotSymbol,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfAnnotExpr {
    pub condition: Box<AnnotExpr>,
    pub then_branch: Box<AnnotExpr>,
    pub else_branch: Option<Box<AnnotExpr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralAnnotExpr {
    Int(IntLiteralAnnotExpr),
    Float(FloatLiteralAnnotExpr),
    Bool(bool),
    Char(char),
    String(String),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntLiteralAnnotExpr {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FloatLiteralAnnotExpr {
    Float32(f32),
    Float64(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructAnnotExpr {
    pub symbol: AnnotSymbol,
    pub fields: Vec<StructFieldAnnotExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructFieldAnnotExpr {
    pub symbol: AnnotSymbol,
    pub value: AnnotExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TupleAnnotExpr {
    pub elements: Vec<AnnotExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryAnnotExpr {
    pub operator: Operator,
    pub value: Box<AnnotExpr>,
}
