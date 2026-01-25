use strum::Display;

use crate::{Spanned, Visibility, ast::{Expr, Symbol, Type}};

pub type Stmt = Spanned<StmtKind>;

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
    pub parameters: Vec<Spanned<FuncParam>>,
    pub body: Expr,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncParam {
    pub symbol: Symbol,
    pub ty: Type,
    pub default_value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclStmt {
    pub visibility: Visibility,
    pub symbol: Symbol,
    pub fields: Vec<Spanned<StructFieldDecl>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructFieldDecl {
    pub visibility: Visibility,
    pub symbol: Symbol,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclStmt {
    pub visibility: Visibility,
    pub symbol: Symbol,
    pub ty: Option<Type>,
    pub initializer: Expr,
}