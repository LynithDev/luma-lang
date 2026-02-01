use std::cell::RefCell;

use luma_diagnostic::LumaError;

use crate::analyzer::{scopes::ScopeManager, symbols::SymbolTable};

pub struct AnalyzerContext {
    pub errors: RefCell<Vec<LumaError>>,
    pub scopes: RefCell<ScopeManager>,
    pub symbols: RefCell<SymbolTable>,
}

impl AnalyzerContext {
    pub fn new() -> Self {
        AnalyzerContext {
            errors: RefCell::new(Vec::new()),
            scopes: RefCell::new(ScopeManager::new()),
            symbols: RefCell::new(SymbolTable::new()),
        }
    }

    #[inline]
    pub fn error(&self, error: LumaError) {
        self.errors.borrow_mut().push(error);
    }
}