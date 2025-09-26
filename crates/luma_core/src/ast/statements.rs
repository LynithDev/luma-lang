use crate::{Cursor, Span};

use crate::{ast::{ConditionalBranch, Expression, Scope, Type, Visibility}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
    pub cursor: Cursor,
}

#[derive(crate::Display, Debug, Clone, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum StatementKind {
    If {
        main_stmt: Box<ConditionalBranch>,
        branches: Option<Vec<ConditionalBranch>>,
        else_stmt: Option<Box<Statement>>,
    },
    While {
        condition: Box<Expression>,
        body: Box<Statement>,
    },
    // For
    Scope(Scope),
    Expression(Expression),
    Continue(Option<String>),
    Break(Option<String>),
    Return(Option<Box<Expression>>),
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
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Box<Type>>,
    pub body: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarDecl {
    pub visibility: Visibility,
    pub name: String,
    pub mutable: bool,
    pub ty: Option<Type>,
    pub value: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassDecl {
    pub visibility: Visibility,
    pub name: String,
    pub fields: Vec<Parameter>,
    pub methods: Vec<FuncDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
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
