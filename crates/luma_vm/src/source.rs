use std::fmt::Display;

use luma_core::{bytecode::Bytecode, CodeSourceKind};

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramSource {
    pub bytecode: Bytecode,
    pub source_kind: CodeSourceKind
}

impl Display for ProgramSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.source_kind.fmt(f)
    }
}

impl ProgramSource {
    pub fn new(source_kind: CodeSourceKind, bytecode: Bytecode) -> Self {
        Self { source_kind, bytecode }
    }
}
