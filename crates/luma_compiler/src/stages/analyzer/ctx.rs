use std::cell::RefCell;

use luma_diagnostic::Diagnostic;

use crate::stages::analyzer::{scopes::ScopeManager, symbols::SymbolTable, type_cache::TypeCache};

pub struct AnalyzerContext {
    pub diagnostics: RefCell<Vec<Diagnostic>>,
    pub scopes: RefCell<ScopeManager>,
    pub symbols: RefCell<SymbolTable>,
    pub type_cache: RefCell<TypeCache>,
}

impl AnalyzerContext {
    pub fn new() -> Self {
        AnalyzerContext {
            diagnostics: RefCell::new(Vec::new()),
            scopes: RefCell::new(ScopeManager::new()),
            symbols: RefCell::new(SymbolTable::new()),
            type_cache: RefCell::new(TypeCache::new()),
        }
    }

    #[inline]
    pub fn diagnostic(&self, diag: Diagnostic) {
        self.diagnostics.borrow_mut().push(diag);
    }
}