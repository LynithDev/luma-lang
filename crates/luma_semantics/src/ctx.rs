use luma_core::ParsedCodeInput;
use luma_diagnostic::Reporter;

use crate::symbol::SymbolTable;

#[derive(Debug)]
pub struct AnalyzerContext<'a> {
    pub reporter: Reporter,
    pub symbol_table: SymbolTable,
    pub input: &'a ParsedCodeInput,
}

impl<'a> AnalyzerContext<'a> {
    pub fn new(parent_reporter: &Reporter, input: &'a ParsedCodeInput) -> Self {
        Self {
            reporter: parent_reporter.with_name(&input.path()),
            symbol_table: SymbolTable::new(),
            input,
        }
    }
}