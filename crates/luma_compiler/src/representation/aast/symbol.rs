use luma_core::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotSymbol {
    pub name: String,
    pub id: usize,
    pub span: Span,
}