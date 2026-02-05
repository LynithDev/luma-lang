use luma_compiler::{AnalyzerStage, CodegenStage, CompilerContext, CompilerStage, LexerStage, ParserStage, aast::AnnotatedAst, ast::Ast, stages::lowering::AstLoweringStage};
use luma_core::CodeSource;
use luma_diagnostic::CompilerResult;

fn main() -> Result<(), ()> {
    // our source code inputs
    let sources = vec![
        CodeSource::from(include_str!("../../../examples/sample.luma")),
    ];
    
    // 0. initialize the compiler context and stages.
    let ctx = CompilerContext::new();
    
    // 1. tokenize the sources
    let tokens = LexerStage::new().process(&ctx, &sources);
    
    // 2. process all of the tokens into ASTs.
    let asts = ParserStage::new().process(&ctx, &tokens);
    error_check(&ctx)?;

    // 3. analyze the ASTs
    let asts = AnalyzerStage::<Ast>::default().process(&ctx, asts);
    error_check(&ctx)?;

    // 4. convert ASTs to AASTs
    let aasts = error_guard(&ctx, AstLoweringStage.process(&ctx, asts))?;

    // 5. analyze the AASTs
    let aasts = AnalyzerStage::<AnnotatedAst>::default().process(&ctx, aasts);
    error_check(&ctx)?;


    println!("Analyzed ASTs: {:#?}", aasts);

    // 6. codegen !!
    let bytecodes = error_guard(&ctx, CodegenStage.process(&ctx, aasts))?;

    println!("Analyzed ASTs: {:#?}", bytecodes);

    Ok(())
}

fn error_check(ctx: &CompilerContext) -> Result<(), ()> {
    let errors = ctx.errors.borrow();
    if errors.is_empty() {
        return Ok(());
    }

    println!("Errors encountered during compilation:");
    for error in errors.iter() {
        println!("{}", error);
    }

    Err(())
}

fn error_guard<T>(ctx: &CompilerContext, result: CompilerResult<T>) -> Result<T, ()> {
    match result {
        Ok(value) => Ok(value),
        Err(err) => {
            ctx.errors.borrow_mut().push(err);
            Err(())
        }
    }
}