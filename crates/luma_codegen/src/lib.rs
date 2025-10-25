use luma_diagnostic::Reporter;
use luma_semantics::ParsedCodeSource;

use crate::ctx::CodegenContext;

pub mod ctx;
pub mod codegen;
pub mod diagnostics;

#[derive(Debug)]
pub struct LumaCodegen<'a> {
    reporter: Reporter,
    entries: Vec<CodegenContext<'a>>,
}

impl<'a> LumaCodegen<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new(reporter: &Reporter) -> Self {
        Self { 
            reporter: reporter.with_name("codegen"),
            entries: Vec::new()
        }
    }

    pub fn add_entry(&mut self, source: &'a ParsedCodeSource) {
        let ctx = CodegenContext::new(&self.reporter,source);
        self.entries.push(ctx);
    }

    pub fn add_entries(&mut self, sources: &'a Vec<ParsedCodeSource>) {
        for source in sources {
            self.add_entry(source);
        }
    }

    pub fn clean(&mut self) {
        self.entries.clear();
    }

    pub fn generate(&mut self) -> bool {
        for ctx in self.entries.iter_mut() {
            if let Err(err) = ctx.generate() {
                ctx.reporter.report(err);
            }
        }

        // return true for success if no errors were reported
        self.reporter.diagnostic_count(luma_diagnostic::DiagnosticKind::Error) == 0
    }
}

