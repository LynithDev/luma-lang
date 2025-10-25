use luma_diagnostic::Reporter;

pub mod hir;

mod source;
pub use source::{ParsedCodeSource, ParsedCodeKind};

pub mod symbol;
mod stages;

mod ctx;
pub use ctx::*;

pub (crate) mod diagnostics;
pub use diagnostics::AnalyzerDiagnostic;

pub struct LumaAnalyzer<'a> {
    pub(crate) reporter: Reporter,
    pub(crate) files: Vec<AnalyzerContext<'a>>,
    pub(crate) stages: Vec<Box<dyn AnalyzerStage>>,
}

impl std::fmt::Debug for LumaAnalyzer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LumaAnalyzer")
            .field("reporter", &self.reporter)
            .field("files", &self.files.len())
            .field("stages", &self.stages.len())
            .finish()
    }
}

pub trait AnalyzerStage {
    fn name(&self) -> &str;

    fn run(&mut self, ctx: &mut AnalyzerContext) -> bool;
}

impl<'a> LumaAnalyzer<'a> {
    pub fn new(parent_reporter: &Reporter) -> Self {
        Self {
            reporter: parent_reporter.with_name("analyzer"),
            files: Vec::new(),
            stages: stages::get_default_stages(),
        }
    }

    pub fn clean_state(&mut self) {
        self.files.clear();
    }

    pub fn add_entry(&mut self, source: &'a ParsedCodeSource) {
        let ctx = AnalyzerContext::new(&self.reporter, source);
        self.files.push(ctx);
    }

    pub fn add_entries(&mut self, sources: &'a Vec<ParsedCodeSource>) {
        for source in sources {
            let ctx = AnalyzerContext::new(&self.reporter, source);
            self.files.push(ctx);
        }
    }

    pub fn analyze(&'a mut self) -> bool {
        // analyze every file with every stage 
        // todo: consider parallelizing this somehow
        // todo: some stages may depend on other stages being run for all files first e.g. after import resolution
        for ctx in self.files.iter_mut() {
            for stage in self.stages.iter_mut() {
                ctx.reporter = self.reporter.with_name(stage.name());

                if !stage.run(ctx) {
                    return false;
                }
            }
        }

        true
    }
}