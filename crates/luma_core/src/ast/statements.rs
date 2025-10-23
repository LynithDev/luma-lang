use crate::ast::AstSymbol;
use crate::{Cursor, Span};

use crate::{ast::expressions::Expression, types::Type, visibility::Visibility};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
    pub cursor: Cursor,
}

#[derive(crate::Display, Debug, Clone, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum StatementKind {
    While {
        label: Option<AstSymbol>,
        condition: Expression,
        body: Expression,
    },
    // For
    Expression {
        inner: Expression
    },
    Continue {
        label: Option<AstSymbol>
    },
    Break {
        label: Option<AstSymbol>
    },
    Return {
        value: Option<Expression>
    },
    Import {
        kind: ImportPropertyKind,
        path: String,
    },

    FuncDecl(FuncDecl),
    VarDecl(VarDecl),
    ClassDecl(ClassDecl),

    EndOfFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncDecl {
    pub visibility: Visibility,
    pub symbol: AstSymbol,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarDecl {
    pub visibility: Visibility,
    pub symbol: AstSymbol,
    pub mutable: bool,
    pub value: Option<Box<Expression>>,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassDecl {
    pub visibility: Visibility,
    pub symbol: AstSymbol,
    pub fields: Vec<Parameter>,
    pub methods: Vec<FuncDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub symbol: AstSymbol,
    pub mutable: bool,
    pub ty: Type,
    pub span: Span,
    pub cursor: Cursor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportPropertyKind {
    All(String),
    Individual(Vec<Renameable>),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Renameable {
    Normally(String),
    Renamed(String, String),
}
