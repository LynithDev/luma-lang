use std::cell::RefCell;

use luma_core::{ast::Ast, CodeSource};

use crate::hir::Hir;

#[derive(Debug, Clone)]
pub struct ParsedCodeSource<'a> {
    pub source: &'a CodeSource,
    pub code: RefCell<ParsedCodeKind>,
}

impl<'a> ParsedCodeSource<'a> {
    pub fn new(source: &'a CodeSource, code: ParsedCodeKind) -> Self {
        Self { source, code: RefCell::new(code) }
    }
}

#[derive(Debug, Clone)]
pub enum ParsedCodeKind {
    Ast(Ast),
    Hir(Hir),
}


impl ParsedCodeKind {
    pub fn as_ast(&self) -> Option<&Ast> {
        match self {
            Self::Ast(ast) => Some(ast),
            _ => None,
        }
    }

    pub fn as_ast_unchecked(&self) -> &Ast {
        match self {
            Self::Ast(ast) => ast,
            _ => panic!("ParsedSourceCode is not an Ast"),
        }
    }

    pub fn as_ast_mut(&mut self) -> Option<&mut Ast> {
        match self {
            Self::Ast(ast) => Some(ast),
            _ => None,
        }
    }

    pub fn as_ast_mut_unchecked(&mut self) -> &mut Ast {
        match self {
            Self::Ast(ast) => ast,
            _ => panic!("ParsedSourceCode is not an Ast"),
        }
    }

    pub fn as_hir(&self) -> Option<&Hir> {
        match self {
            Self::Hir(hir) => Some(hir),
            _ => None,
        }
    }

    pub fn as_hir_unchecked(&self) -> &Hir {
        match self {
            Self::Hir(hir) => hir,
            _ => panic!("ParsedSourceCode is not a Hir"),
        }
    }

    pub fn as_hir_mut(&mut self) -> Option<&mut Hir> {
        match self {
            Self::Hir(hir) => Some(hir),
            _ => None,
        }
    }

    pub fn as_hir_mut_unchecked(&mut self) -> &mut Hir {
        match self {
            Self::Hir(hir) => hir,
            _ => panic!("ParsedSourceCode is not a Hir"),
        }
    }
}