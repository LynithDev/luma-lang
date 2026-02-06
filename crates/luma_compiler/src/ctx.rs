use std::cell::{Ref, RefCell, RefMut};

use luma_core::SourceManager;
use luma_diagnostic::LumaError;

pub struct CompilerContext {
    pub sources: SourceManager,
    current_stage_name: RefCell<String>,
    errors: RefCell<Vec<LumaError>>,
}

pub type CodeSourceId = u32;

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            sources: SourceManager::new(),
            current_stage_name: RefCell::new(String::new()),
            errors: RefCell::new(Vec::new()),
        }
    }

    pub fn add_error(&self, error: LumaError) {
        self.errors.borrow_mut().push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.borrow().is_empty()
    }

    pub fn get_errors(&self) -> Ref<'_, Vec<LumaError>> {
        self.errors.borrow()
    }

    pub(crate) fn get_errors_mut(&self) -> RefMut<'_, Vec<LumaError>> {
        self.errors.borrow_mut()
    }

    pub fn set_stage_name(&self, stage_name: &str) {
        *self.current_stage_name.borrow_mut() = stage_name.to_string();
    }

    pub fn get_stage_name(&self) -> Ref<'_, String> {
        self.current_stage_name.borrow()
    }
}