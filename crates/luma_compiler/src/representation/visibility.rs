use luma_core::Span;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Visibility {
    pub kind: VisibilityKind,
    pub span: Option<Span>,
}

impl Visibility {
    pub fn unspanned(kind: VisibilityKind) -> Self {
        Self { kind, span: None }
    }

    pub fn spanned(span: Span, kind: VisibilityKind) -> Self {
        Self { kind, span: Some(span) }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisibilityKind {
    /// visible to all
    /// `pub`
    Public,
    
    /// visible only within the defining scope
    /// no `pub` keyword or `pub(this)`
    #[default]
    Private,

    /// visible within the same module
    /// `pub(module)`
    Module,
}

impl VisibilityKind {
    #[must_use]
    pub const fn is_public(&self) -> bool {
        matches!(self, VisibilityKind::Public)
    }

    #[must_use]
    pub const fn is_private(&self) -> bool {
        matches!(self, VisibilityKind::Private)
    }

    #[must_use]
    pub const fn is_module_private(&self) -> bool {
        matches!(self, VisibilityKind::Module)
    }
}