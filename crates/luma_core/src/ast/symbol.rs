use strum::Display;

use crate::span::Spanned;

pub type Symbol = Spanned<SymbolKind>;

#[derive(Display, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    #[strum(to_string = "Named({name})")]
    Named {
        name: String,
    },
    #[strum(to_string = "Identified({name}, {id})")]
    Identified {
        name: String,
        id: usize,
    },
}

impl SymbolKind {
    #[must_use]
    pub const fn named(name: String) -> Self {
        SymbolKind::Named { name }
    }

    #[must_use]
    pub const fn identified(name: String, id: usize) -> Self {
        SymbolKind::Identified { name, id }
    }

    pub fn with_id(self, id: usize) -> Self {
        match self {
            SymbolKind::Named { name } => SymbolKind::Identified { name, id },
            SymbolKind::Identified { name, .. } => SymbolKind::Identified { name, id },
        }
    }

    pub fn set_id(&mut self, id: usize) {
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

    pub fn id(&self) -> Option<usize> {
        match self {
            SymbolKind::Identified { id, .. } => Some(*id),
            SymbolKind::Named { .. } => None,
        }
    }

    pub fn is_named(&self) -> bool {
        matches!(self, SymbolKind::Named { .. })
    }

    pub fn is_identified(&self) -> bool {
        matches!(self, SymbolKind::Identified { .. })
    }
}