use luma_core::SymbolId;
use luma_core::{Cursor, Display, Span};

use luma_core::{types::Type, visibility::Visibility};

use crate::hir::expressions::HirExpression;

#[derive(Debug, Clone, PartialEq)]
pub struct HirStatement {
    pub kind: HirStatementKind,
    pub span: Span,
    pub cursor: Cursor,
}

#[derive(Display, Debug, Clone, PartialEq)]
#[display(case = "snake_case")]
pub enum HirStatementKind {
    Loop {
        symbol_id: SymbolId,
        body: Box<HirStatement>,
    },
    Expression {
        inner: HirExpression
    },
    Continue {
        label: Option<SymbolId>
    },
    Break {
        label: Option<SymbolId>
    },
    Return {
        value: Option<Box<HirExpression>>
    },
    Import {
        kind: HirImportPropertyKind,
        path: String,
    },

    FuncDecl(HirFuncDecl),
    VarDecl(HirVarDecl),
    ClassDecl(HirClassDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirFuncDecl {
    pub visibility: Visibility,
    pub symbol_id: SymbolId,
    pub parameters: Vec<HirParameter>,
    pub return_type: Type,
    pub body: Option<Box<HirExpression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirVarDecl {
    pub visibility: Visibility,
    pub symbol_id: SymbolId,
    pub mutable: bool,
    pub ty: Type,
    pub value: Option<Box<HirExpression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirClassDecl {
    pub visibility: Visibility,
    pub symbol_id: SymbolId,
    pub fields: Vec<HirParameter>,
    pub methods: Vec<HirFuncDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirParameter {
    pub symbol_id: SymbolId,
    pub mutable: bool,
    pub ty: Type,
    pub span: Span,
    pub cursor: Cursor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HirImportPropertyKind {
    All(String),
    Individual(Vec<HirRenameable>),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HirRenameable {
    Normally(String),
    Renamed(String, String),
}