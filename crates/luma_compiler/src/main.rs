use luma_compiler::{AnalyzerStage, CompilerContext, CompilerStage, LexerStage, ParserStage};
use luma_core::CodeSource;

fn main() {
    
    // our source code inputs
    let sources = vec![
        CodeSource::from(include_str!("../../../examples/sample.luma")),
    ];
    
    // 0. initialize the compiler context and stages.
    let ctx = CompilerContext::new();
    let lexer = LexerStage::new();
    let parser = ParserStage::new();
    let analyzer = AnalyzerStage::default();
    
    // 1. tokenize the sources
    let tokens = lexer.process(&ctx, &sources);
    
    // 2. process all of the tokens into ASTs.
    let asts = parser.process(&ctx, &tokens);

    // check for errors
    if !ctx.errors.borrow().is_empty() {
        println!("Errors encountered during compilation:");
        for error in ctx.errors.borrow().iter() {
            println!("{}", error);
        }
        return;
    }

    // 3. analyze the ASTs
    let asts = analyzer.process(&ctx, asts);

    // check for errors
    if !ctx.errors.borrow().is_empty() {
        println!("Errors encountered during compilation:");
        for error in ctx.errors.borrow().iter() {
            println!("{}", error);
        }
        return;
    }


    println!("Analyzed ASTs: {:#?}", asts);

}