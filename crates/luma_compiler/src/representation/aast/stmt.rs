use strum::Display;

use luma_core::Span;

use crate::{Type, Visibility, aast::*, ScopeId};

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotStmt {
    pub item: AnnotStmtKind,
    pub scope_id: ScopeId,
    pub span: Span,
}

impl AnnotStmt {
    pub fn new(span: Span, item: AnnotStmtKind, scope_id: ScopeId) -> Self {
        AnnotStmt { 
            item,
            scope_id,
            span
        }
    }
}

#[derive(Display, Debug, Clone, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum AnnotStmtKind {
    Expr(AnnotExpr),
    Func(FuncDeclAnnotStmt),
    Return(ReturnAnnotStmt),
    Struct(StructDeclAnnotStmt),
    Var(VarDeclAnnotStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncDeclAnnotStmt {
    pub visibility: Visibility,
    pub symbol: AnnotSymbol,
    pub parameters: Vec<AnnotFuncParam>,
    pub body: AnnotExpr,
    pub return_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotFuncParam {
    pub symbol: AnnotSymbol,
    pub ty: Type,
    pub default_value: Option<AnnotExpr>,
    pub span: Span,
    pub scope_id: ScopeId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnAnnotStmt {
    pub value: Option<AnnotExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclAnnotStmt {
    pub visibility: Visibility,
    pub symbol: AnnotSymbol,
    pub fields: Vec<StructFieldAnnotDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructFieldAnnotDecl {
    pub visibility: Visibility,
    pub symbol: AnnotSymbol,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclAnnotStmt {
    pub visibility: Visibility,
    pub symbol: AnnotSymbol,
    pub ty: Type,
    pub initializer: AnnotExpr,
}