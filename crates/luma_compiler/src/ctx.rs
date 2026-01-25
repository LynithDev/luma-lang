use std::cell::RefCell;

use luma_diagnostic::LumaError;

pub struct CompilerContext {
    pub errors: RefCell<Vec<LumaError>>,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            errors: RefCell::new(Vec::new()),
        }
    }
}