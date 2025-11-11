use std::{collections::HashMap, rc::Rc};

use luma_core::CodeSourceKind;

use crate::{DiagnosticKind, DiagnosticReport};

pub type ReporterName = Rc<String>;

#[derive(Default)]
pub struct DiagnosticStore {
    pub (crate) diagnostics: HashMap<String, Vec<DiagnosticEntry>>,
    pub (crate) kind_count: HashMap<DiagnosticKind, usize>,
}

#[derive(Debug)]
pub struct DiagnosticEntry {
    pub (crate) reporter_name: ReporterName,
    pub (crate) diagnostic: DiagnosticReport,
}

impl std::fmt::Debug for DiagnosticStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiagnosticStore")
            .field("diagnostics", &self.diagnostics.len())
            .field("kind_count", &self.kind_count)
            .finish()
    }
}

unsafe impl Send for DiagnosticStore {}
unsafe impl Sync for DiagnosticStore {}

impl DiagnosticStore {
    pub fn new() -> Self {
        DiagnosticStore::default()
    }

    pub fn report(&mut self, source: &CodeSourceKind, reporter_name: ReporterName, diagnostic: DiagnosticReport) {
        *self.kind_count.entry(diagnostic.message.kind()).or_default() += 1;
        self.diagnostics.entry(source.source_name()).or_default().push(DiagnosticEntry {
            reporter_name,
            diagnostic,
        });
    }

    pub fn is_clean(&self, source_file: &str) -> bool {
        self.diagnostics.get(source_file).map(|d| d.is_empty()).unwrap_or(true)
    }

    pub fn diagnostic_count(&self, kind: DiagnosticKind) -> usize {
        self.kind_count.get(&kind).cloned().unwrap_or(0)
    }
}
