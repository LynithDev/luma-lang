use luma_core::ParsedCodeInput;
use luma_diagnostic::{LumaResult, Reporter};

pub mod symbol;
mod passes;

mod ctx;
pub use ctx::*;

pub (crate) mod diagnostics;
pub use diagnostics::AnalyzerDiagnostic;

use crate::passes::SymbolTableBuildingPass;

#[derive(Debug)]
pub struct LumaAnalyzer<'a> {
    pub(crate) reporter: Reporter,
    pub(crate) files: Vec<AnalyzerContext<'a>>,
}

impl<'a> LumaAnalyzer<'a> {
    pub fn new(parent_reporter: &Reporter) -> Self {
        Self {
            reporter: parent_reporter.with_name("analyzer"),
            files: Vec::new(),
        }
    }

    pub fn clean_state(&mut self) {
        self.files.clear();
    }

    pub fn add_entry(&mut self, input: &'a ParsedCodeInput) {
        let ctx = AnalyzerContext::new(&self.reporter, input);
        self.files.push(ctx);
    }

    pub fn add_entries(&mut self, input: &'a Vec<ParsedCodeInput>) {
        for entry in input {
            let ctx = AnalyzerContext::new(&self.reporter, entry);
            self.files.push(ctx);
        }
    }

    pub fn analyze(&'a mut self) -> LumaResult<()> {
        // Symbol table building
        for ctx in &mut self.files {
            SymbolTableBuildingPass::run(ctx)?;
            dbg!(&ctx.symbol_table);

            dbg!(ctx.symbol_table.lookup("test"));
        }

        Ok(())
    }
}
