use luma_core::{CodeSource, CodeSourceId};
use luma_diagnostic::LumaError;

use crate::{AnalyzerStage, AstLoweringStage, CodegenStage, CompilerContext, CompilerStage, LexerStage, ParserStage, aast::AnnotatedAst, ast::Ast};

pub struct LumaCompiler {
    // todo: flags, options
}

#[derive(Debug)]
pub struct CompileResult {
    pub errors: Vec<LumaError>,
}

impl LumaCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(self, sources: impl IntoIterator<Item = CodeSource>) -> CompileResult {
        let mut ctx = CompilerContext::new();
        let mut source_ids = Vec::<CodeSourceId>::new();

        for source in sources {
            source_ids.push(ctx.sources.add_source(source));
        }

        let _ = self.run_pipeline(&ctx, source_ids);

        let errors = std::mem::take(&mut *ctx.get_errors_mut());

        CompileResult {
            errors,
        }
    }

    fn run_pipeline(&self, ctx: &CompilerContext, source_ids: Vec<CodeSourceId>) -> Result<(), ()> {
        let tokens = run_stage(ctx, LexerStage, source_ids)?;
        let asts = run_stage(ctx, ParserStage, &tokens)?;
        let asts = run_stage(ctx, AnalyzerStage::<Ast>::default(), asts)?;
        println!("{:#?}", asts);
        let aasts = run_stage(ctx, AstLoweringStage, asts)?;
        let aasts = run_stage(ctx, AnalyzerStage::<AnnotatedAst>::default(), aasts)?;
        let bytecodes = run_stage(ctx, CodegenStage, aasts)?;

        Ok(())
    }

    
}

pub(crate) fn run_stage<'stage, S>(
    ctx: &CompilerContext,
    stage: S,
    input: S::Input,
) -> Result<S::Output, ()>
where
    S: CompilerStage<'stage>,
{
    ctx.set_stage_name(S::name());
    let output = stage.process(ctx, input);

    if ctx.has_errors() {
        return Err(());
    }

    Ok(output)
}