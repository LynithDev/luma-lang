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
pub struct AnalyzerStage<Input> {
    passes: Vec<Box<dyn AnalyzerPass<Input>>>,
    ctx: AnalyzerContext,
}

impl<Input> AnalyzerStage<Input> {
    pub fn new() -> Self {
        AnalyzerStage {
            passes: Vec::new(),
            ctx: AnalyzerContext::new(),
        }
    }

    pub fn with_passes<I>(passes: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn AnalyzerPass<Input> + 'static>>,
    {
        let mut this = Self::new();

        for pass in passes {
            this.passes.push(pass);
        }

        this
    }

    pub fn add_pass<A>(&mut self, pass: A)
    where A: AnalyzerPass<Input> + 'static,
    {
        self.passes.push(Box::new(pass));
    }

    pub fn add_passes<I, A>(&mut self, passes: I)
    where
        I: IntoIterator<Item = A>,
        A: AnalyzerPass<Input> + 'static,
    {
        for pass in passes {
            self.passes.push(Box::new(pass));
        }
    }
}

impl<Input> CompilerStage<'_> for AnalyzerStage<Input> {
    type Input = Vec<Input>;
    type Output = Vec<Input>;

    fn name() -> &'static str {
        "analyzer"
    }

    fn process(mut self, ctx: &CompilerContext, input: Self::Input) -> Self::Output {
        let mut asts = input;

        // todo: somehow make this faster (parallelize?)
        for ast in &mut asts {
            for analyzer in self.passes.iter() {
                tracing::debug!("running analyzer stage: '{}'", analyzer.name());

                analyzer.analyze(&mut self.ctx, ast);
            }
        }

        ctx.get_errors_mut().append(&mut self.ctx.errors.borrow_mut());

        asts
    }
}

pub trait AnalyzerPass<Input> {
    fn name(&self) -> String;

    fn analyze(&self, ctx: &mut AnalyzerContext, input: &mut Input);
}
