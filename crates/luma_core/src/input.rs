use std::{hash::Hash, path::PathBuf};

use crate::ast::Ast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeInputKind {
    File(PathBuf),
    Virtual,
}

impl std::fmt::Display for CodeInputKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeInputKind::File(path) => write!(f, "{}", path.display()),
            CodeInputKind::Virtual => write!(f, "<virtual>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeInput {
    pub(crate) kind: CodeInputKind,
    pub(crate) source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCodeInput {
    pub(crate) kind: CodeInputKind,
    pub(crate) source: String,
    pub ast: Ast,
}


macro_rules! impl_code_input {
    ($name:ident) => {
        impl Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                match self.kind {
                    CodeInputKind::File(ref path) => path.hash(state),
                    CodeInputKind::Virtual => self.source.hash(state),
                }
            }
        }

        impl $name {
            pub fn path(&self) -> String {
                if let CodeInputKind::File(path) = &self.kind {
                    path.to_string_lossy().to_string()
                } else {
                    "<virtual>".to_string()
                }
            }

            pub fn kind(&self) -> &CodeInputKind {
                &self.kind
            }

            pub fn source(&self) -> &str {
                &self.source
            }
        }
    };
}

impl_code_input!(CodeInput);
impl_code_input!(ParsedCodeInput);

impl TryFrom<PathBuf> for CodeInput {
    type Error = std::io::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let source = std::fs::read_to_string(&path)?;
        Ok(Self { 
            source,
            kind: CodeInputKind::File(path),
        })
    }
}

impl From<&str> for CodeInput {
    fn from(source: &str) -> Self {
        Self { 
            kind: CodeInputKind::Virtual, 
            source: source.trim().to_string(),
        }
    }
}

impl CodeInput {
    pub fn with_ast(self, ast: Ast) -> ParsedCodeInput {
        ParsedCodeInput { 
            kind: self.kind,
            source: self.source,
            ast
        }
    }
}

impl ParsedCodeInput {
    pub fn from_input(input: CodeInput, ast: Ast) -> Self {
        Self {
            kind: input.kind,
            source: input.source,
            ast,
        }
    }
}
