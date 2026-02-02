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
pub struct AnalyzerStage {
    passes: Vec<Box<dyn AnalyzerPass>>,
    ctx: AnalyzerContext,
}

impl AnalyzerStage {
    pub fn new() -> Self {
        AnalyzerStage {
            passes: Vec::new(),
            ctx: AnalyzerContext::new(),
        }
    }

    pub fn add_pass<A>(&mut self, pass: A)
    where A: AnalyzerPass + 'static,
    {
        self.passes.push(Box::new(pass));
    }

    pub fn add_passes<I, A>(&mut self, passes: I)
    where
        I: IntoIterator<Item = A>,
        A: AnalyzerPass + 'static,
    {
        for pass in passes {
            self.passes.push(Box::new(pass));
        }
    }
}

impl Default for AnalyzerStage {
    fn default() -> Self {
        let mut this = Self::new();

        for pass in passes::default_passes() {
            this.passes.push(pass);
        }

        this
    }
}

impl CompilerStage<'_> for AnalyzerStage {
    type Input = Vec<Ast>;
    type Output = Vec<Ast>;

    fn name() -> &'static str {
        "analyzer"
    }

    fn process(mut self, ctx: &CompilerContext, input: Self::Input) -> Self::Output {
        let mut asts = input;

        // todo: somehow make this faster (parallelize?)
        for ast in &mut asts {
            for analyzer in self.passes.iter_mut() {
                tracing::debug!("running analyzer stage: '{}'", analyzer.name());

                analyzer.analyze(&mut self.ctx, ast);
            }
        }

        ctx.errors
            .borrow_mut()
            .append(&mut self.ctx.errors.borrow_mut());

        asts
    }
}

pub trait AnalyzerPass {
    fn name(&self) -> String;

    fn analyze(&mut self, ctx: &mut AnalyzerContext, input: &mut Ast);
}
