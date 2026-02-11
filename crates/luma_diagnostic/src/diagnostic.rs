use luma_core::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub title: String,
    pub annotation: Option<String>,
    pub span: Option<Span>,
    pub additional_contexts: Vec<DiagnosticContext>,

    #[cfg(debug_assertions)]
    pub thrower: CallerInfo,
}

impl Diagnostic {
    pub fn span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn maybe_span(mut self, span: Option<Span>) -> Self {
        self.span = span;
        self
    }

    pub fn context(mut self, context: DiagnosticContext) -> Self {
        self.additional_contexts.push(context);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticContext {
    pub kind: DiagnosticContextKind,
    pub annotation: Option<String>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, Copy,PartialEq, Eq)]
pub enum DiagnosticContextKind {
    Primary,
    Context,
    Unannotated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallerInfo {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

pub trait AsDiagnostic {
    fn level(&self) -> DiagnosticLevel;
    fn title(&self) -> String;
    fn annotation(&self) -> Option<String>;
}

pub trait AsDiagnosticContext {
    fn kind(&self) -> DiagnosticContextKind;
    fn annotation(&self) -> Option<String>;
}