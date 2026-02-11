use luma_core::{CodeSource, CodeSourceId, SourceManager};
use luma_diagnostic::Diagnostic;

use crate::{AnalyzerStage, AstLoweringStage, CodegenStage, CompilerContext, CompilerOptions, CompilerStage, LexerStage, ParserStage, aast::AnnotatedAst, ast::Ast, bytecode::ModuleBytecode};

pub struct LumaCompiler {
    options: CompilerOptions,
}

#[derive(Debug)]
pub struct CompileResult {
    pub sources: SourceManager,
    pub diagnostics: Vec<Diagnostic>,
    pub result: Option<Vec<ModuleBytecode>>
}

impl LumaCompiler {
    pub fn new() -> Self {
        Self {
            options: CompilerOptions::default(),
        }
    }

    pub fn configure(options: CompilerOptions) -> Self {
        Self { 
            options
        }
    }

    pub fn compile(self, sources: impl IntoIterator<Item = CodeSource>) -> CompileResult {
        let mut ctx = CompilerContext::configure(self.options);
        let mut source_ids = Vec::<CodeSourceId>::new();

        for source in sources {
            source_ids.push(ctx.sources.add_source(source));
        }

        let bytecode = Self::run_pipeline(&ctx, source_ids);

        CompileResult {
            sources: ctx.sources,
            diagnostics: ctx.diagnostics.into_inner(),
            result: bytecode.ok(),
        }
    }

    fn run_pipeline(ctx: &CompilerContext, source_ids: Vec<CodeSourceId>) -> Result<Vec<ModuleBytecode>, ()> {
        let tokens = run_stage(ctx, LexerStage, source_ids)?;
        let asts = run_stage(ctx, ParserStage, &tokens)?;
        let asts = run_stage(ctx, AnalyzerStage::<Ast>::default(), asts)?;
        let aasts = run_stage(ctx, AstLoweringStage, asts)?;
        let aasts = run_stage(ctx, AnalyzerStage::<AnnotatedAst>::default(), aasts)?;
        let bytecodes = run_stage(ctx, CodegenStage, aasts)?;

        Ok(bytecodes)
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

    if ctx.has_diagnostics() {
        return Err(());
    }

    Ok(output)
}