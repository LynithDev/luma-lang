use crate::ast::Ast;

use crate::{CompilerContext, CompilerStage};

// analyzer modules
mod ctx;
pub use ctx::*;

pub mod scopes;
pub mod registry;
pub mod symbols;
pub mod passes;
pub mod error;

/// Responsible for things like name resolution, type checking, and other semantic analyses.
pub struct Analyzer {
    asts: Vec<Ast>,
    analyzers: Vec<Box<dyn AnalyzerStage>>,
    ctx: AnalyzerContext,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {
            asts: Vec::new(),
            analyzers: Vec::new(),
            ctx: AnalyzerContext::new(),
        }
    }

    pub fn add_analyzer<A: AnalyzerStage + 'static>(&mut self, analyzer: A) {
        self.analyzers.push(Box::new(analyzer));
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        let mut this = Self::new();

        for analyzer in passes::default_passes() {
            this.analyzers.push(analyzer);
        }

        this
    }
}

impl CompilerStage for Analyzer {
    type Input = Ast;
    type ProcessedOutput = Vec<Ast>;
    type ErrorKind = ();

    fn name() -> String {
        String::from("analyzer")
    }

    fn feed(&mut self, _input: Self::Input) {
        self.asts.push(_input);
    }

    fn process(mut self, ctx: &CompilerContext) -> Self::ProcessedOutput {
        let mut asts = self.asts;

        // todo: somehow make this faster (parallelize?)
        for ast in &mut asts {
            for analyzer in self.analyzers.iter_mut() {
                println!("Running analyzer stage: {}", analyzer.name());

                analyzer.analyze(&self.ctx, ast);

                ctx.errors
                    .borrow_mut()
                    .append(&mut self.ctx.errors.borrow_mut());
            }
        }

        asts
    }
}

pub trait AnalyzerStage {
    fn name(&self) -> String;

    fn analyze(&mut self, ctx: &AnalyzerContext, input: &mut Ast);
}
