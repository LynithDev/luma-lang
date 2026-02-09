use strum::Display;

use luma_core::Span;

use crate::{Type, Visibility, ast::*};

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub item: StmtKind,
    pub scope_id: Option<usize>,
    pub span: Span,
}

impl Stmt {
    pub fn new(span: Span, item: StmtKind) -> Self {
        Stmt { 
            item,
            scope_id: None,
            span
        }
    }

    pub fn set_scope_id(&mut self, scope_id: usize) {
        self.scope_id = Some(scope_id);
    }
}

#[derive(Display, Debug, Clone, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum StmtKind {
    Expr(Expr),
    Func(FuncDeclStmt),
    Return(ReturnStmt),
    Struct(StructDeclStmt),
    Var(VarDeclStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncDeclStmt {
    pub visibility: Visibility,
    pub symbol: Symbol,
    pub parameters: Vec<FuncParam>,
    pub body: Expr,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncParam {
    pub symbol: Symbol,
    pub ty: Type,
    pub default_value: Option<Expr>,
    pub span: Span,
    pub scope_id: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclStmt {
    pub visibility: Visibility,
    pub symbol: Symbol,
    pub fields: Vec<StructDeclField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclField {
    pub visibility: Visibility,
    pub symbol: Symbol,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclStmt {
    pub visibility: Visibility,
    pub symbol: Symbol,
    pub ty: Option<Type>,
    pub initializer: Expr,
}