use luma_core::Span;
use strum::Display;

use crate::{Operator, TypeKind, ast::*};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub item: ExprKind,
    pub ty: Option<TypeKind>,
    pub scope_id: Option<usize>,
    pub span: Span,
}

impl Expr {
    pub fn new(span: Span, item: ExprKind) -> Self {
        Self {
            item,
            ty: None,
            scope_id: None,
            span,
        }
    }

    pub fn set_type(&mut self, ty: TypeKind) {
        self.ty = Some(ty);
    }

    pub fn set_scope(&mut self, scope_id: usize) {
        self.scope_id = Some(scope_id);
    }
}

#[derive(Display, Debug, Clone, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum ExprKind {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Block(BlockExpr),
    Call(CallExpr),
    Get(GetExpr),
    Group(Box<Expr>),
    Ident(IdentExpr),
    If(IfExpr),
    Literal(LiteralExpr),
    Struct(StructExpr),
    TupleLiteral(TupleExpr),
    Unary(UnaryExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignExpr {
    pub target: Box<Expr>,
    pub operator: Operator,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Operator,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockExpr {
    pub statements: Vec<Stmt>,
    pub tail_expr: Option<Box<Expr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetExpr {
    pub object: Box<Expr>,
    pub property: Symbol,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentExpr {
    pub symbol: SymbolKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub condition: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Option<Box<Expr>>,
}

#[derive(Display, Debug, Clone, PartialEq)]
pub enum LiteralExpr {
    Int(u64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructExpr {
    pub symbol: Symbol,
    pub fields: Vec<StructExprField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructExprField {
    pub symbol: Symbol,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TupleExpr {
    pub elements: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: Operator,
    pub value: Box<Expr>,
}
