use std::cell::RefCell;

use luma_diagnostic::LumaError;

use crate::{analyzer::symbols::SymbolTable};

pub struct AnalyzerContext {
    pub symbols: RefCell<SymbolTable>,
    pub errors: RefCell<Vec<LumaError>>,
}

impl AnalyzerContext {
    pub fn new() -> Self {
        AnalyzerContext {
            symbols: RefCell::new(SymbolTable::new()),
            errors: RefCell::new(Vec::new()),
        }
    }

    #[inline]
    pub fn error(&self, error: LumaError) {
        self.errors.borrow_mut().push(error);
    }
}