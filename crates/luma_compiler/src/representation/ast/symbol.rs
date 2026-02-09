use std::ops::{Deref, DerefMut};

use luma_core::Span;
use strum::Display;

use crate::SymbolId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub span: Span,
}

impl Symbol {
    #[must_use]
    pub const fn new(span: Span, kind: SymbolKind) -> Self {
        Self { kind, span }
    }
}

impl Deref for Symbol {
    type Target = SymbolKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl DerefMut for Symbol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}

#[derive(Display, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    #[strum(to_string = "Named({name})")]
    Named {
        name: String,
    },
    #[strum(to_string = "Identified({name}, {id})")]
    Identified {
        name: String,
        id: SymbolId,
    },
}

impl SymbolKind {
    #[must_use]
    pub const fn named(name: String) -> Self {
        SymbolKind::Named { name }
    }

    #[must_use]
    pub const fn identified(name: String, id: SymbolId) -> Self {
        SymbolKind::Identified { name, id }
    }

    pub fn with_id(self, id: SymbolId) -> Self {
        match self {
            SymbolKind::Named { name } => SymbolKind::Identified { name, id },
            SymbolKind::Identified { name, .. } => SymbolKind::Identified { name, id },
        }
    }

    pub fn set_id(&mut self, id: SymbolId) {
        match self {
            SymbolKind::Named { name } => {
                *self = SymbolKind::Identified { name: name.clone(), id };
            }
            SymbolKind::Identified { name, .. } => {
                *self = SymbolKind::Identified { name: name.clone(), id };
            }
        }
    }

    pub fn name(&self) -> &str {
        match self {
            SymbolKind::Named { name } => name,
            SymbolKind::Identified { name, .. } => name,
        }
    }

    pub fn id(&self) -> Option<SymbolId> {
        match self {
            SymbolKind::Identified { id, .. } => Some(*id),
            SymbolKind::Named { .. } => None,
        }
    }

    pub fn unwrap_id(&self) -> usize {
        self.id().unwrap_or_else(|| panic!("symbol '{}' does not have an id", self.name()))
    }

    pub fn is_named(&self) -> bool {
        matches!(self, SymbolKind::Named { .. })
    }

    pub fn is_identified(&self) -> bool {
        matches!(self, SymbolKind::Identified { .. })
    }
}