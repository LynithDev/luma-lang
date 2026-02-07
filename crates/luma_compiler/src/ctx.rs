use std::cell::{Ref, RefCell, RefMut};

use luma_core::SourceManager;
use luma_diagnostic::Diagnostic;

pub struct CompilerContext {
    pub sources: SourceManager,
    current_stage_name: RefCell<String>,
    pub(crate) diagnostics: RefCell<Vec<Diagnostic>>,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            sources: SourceManager::new(),
            current_stage_name: RefCell::new(String::new()),
            diagnostics: RefCell::new(Vec::new()),
        }
    }

    pub fn add_diag(&self, diag: Diagnostic) {
        self.diagnostics.borrow_mut().push(diag);
    }

    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.borrow().is_empty()
    }

    pub fn get_diagnostics(&self) -> Ref<'_, Vec<Diagnostic>> {
        self.diagnostics.borrow()
    }

    pub(crate) fn get_diagnostics_mut(&self) -> RefMut<'_, Vec<Diagnostic>> {
        self.diagnostics.borrow_mut()
    }

    pub fn set_stage_name(&self, stage_name: &str) {
        *self.current_stage_name.borrow_mut() = stage_name.to_string();
    }

    pub fn get_stage_name(&self) -> Ref<'_, String> {
        self.current_stage_name.borrow()
    }
}