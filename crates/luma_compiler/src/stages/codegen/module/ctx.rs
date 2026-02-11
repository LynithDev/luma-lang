use std::cell::RefCell;

use crate::stages::codegen::stores::FunctionTable;

#[derive(Debug)]
pub struct ModuleContext {
    /// function table for the current module
    pub function_table: RefCell<FunctionTable>,
    // struct table for the current module
    // TODO: implement struct table
}

impl ModuleContext {
    pub fn new() -> Self {
        Self {
            function_table: RefCell::new(FunctionTable::new()),
        }
    }
}