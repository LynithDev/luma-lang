use luma_diagnostic::Reporter;

use crate::{symbol::SymbolsTable, ParsedCodeSource};

#[derive(Debug)]
pub struct AnalyzerContext<'a> {
    pub reporter: Reporter,
    pub symbol_table: SymbolsTable,
    pub input: &'a ParsedCodeSource<'a>,
}

impl<'a> AnalyzerContext<'a> {
    pub fn new(parent_reporter: &Reporter, input: &'a ParsedCodeSource<'a>) -> Self {
        Self {
            reporter: parent_reporter.clone(),
            symbol_table: SymbolsTable::new(),
            input,
        }
    }
}
