use crate::stages::codegen::stores::{ConstantTable, ExportTable, FunctionTable};

#[derive(Debug)]
pub struct ModuleContext {
    pub export_table: ExportTable,
    pub function_table: FunctionTable,
    pub constant_table: ConstantTable,

    // TODO: implement struct table
}

impl ModuleContext {
    pub fn new() -> Self {
        Self {
            export_table: ExportTable::new(),
            function_table: FunctionTable::new(),
            constant_table: ConstantTable::new(),
        }
    }
}