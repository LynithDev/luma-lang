use std::{rc::Rc, sync::{Arc, Mutex}};

use luma_core::CodeSourceKind;
use luma_diagnostic::{Diagnostic, DiagnosticKind, DiagnosticReport, DiagnosticStore};

pub type VmResult<T> = Result<T, VmError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmExitResult {
    pub code: VmExitCode,
    pub error: Option<VmError>,
}

pub type VmExitCode = i32;

impl VmExitResult {
    pub fn from_code(code: VmExitCode) -> Self {
        Self { code, error: None }
    }

    pub fn from_error(error: VmError) -> Self {
        Self { 
            code: -1, 
            error: Some(error)
        }
    }
}

#[derive(Debug, Clone, Diagnostic, PartialEq, Eq)]
pub enum VmError {
    #[error("no entrypoint provided to VM")]
    NoEntrypoint,
    #[error("recursive max call depth ({0}) exceeded")]
    MaxCallDepthExceeded(usize),
    #[error("stack underflow")]
    StackUnderflow,
    #[error("stack overflow")]
    StackOverflow,
    #[error("max frame count exceeded")]
    MaxFrameCountExceeded,
    #[error("no active call frame")]
    NoActiveCallFrame,
    #[error("null reference encountered")]
    NullReference,
    #[error("type mismatch during operation")]
    TypeMismatch,
}

pub struct Reporter {
    source: CodeSourceKind,
    inner: Arc<Mutex<DiagnosticStore>>,
    name: Rc<String>,
}

impl std::fmt::Debug for Reporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reporter")
            .field("name", &self.name)
            .field("source", &self.source)
            .finish()
    }
}

impl Reporter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Reporter {
            name: Rc::new("VM".to_string()),
            source: CodeSourceKind::Virtual,
            inner: Arc::new(Mutex::new(DiagnosticStore::new())),
        }
    }

    pub fn source(&self) -> &CodeSourceKind {
        &self.source
    }

    pub fn report(&self, diagnostic: DiagnosticReport) {
        let mut guard = self.lock();
        guard.report(&self.source, Rc::clone(&self.name), diagnostic);
    }

    pub fn report_all(&self, diagnostics: Vec<DiagnosticReport>) {
        let mut guard = self.lock();
        for diagnostic in diagnostics {
            guard.report(&self.source, Rc::clone(&self.name), diagnostic);
        }
    }

    pub fn is_clean(&self) -> bool {
        let guard = self.lock();
        guard.is_clean(&self.name)
    }

    pub fn diagnostic_count(&self, kind: DiagnosticKind) -> usize {
        let guard = self.lock();
        guard.diagnostic_count(kind)
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, DiagnosticStore> {
        self.inner.lock().expect("couldn't lock reporter mutex")
    }
}

