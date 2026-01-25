use luma_compiler::{Analyzer, CompilerContext, CompilerStage, Lexer, Parser};
use luma_core::CodeSource;

fn main() {
    
    // our source code inputs
    let sources = vec![
        CodeSource::from(include_str!("../../../examples/sample.luma")),
    ];
    
    // initialize the compiler context and stages.
    let ctx = CompilerContext::new();
    let mut lexer = Lexer::new();
    let mut parser = Parser::new();
    let mut analyzer = Analyzer::default();
    

    // We iterate through each source code and input it into the lexer.
    // We process all of the sources into tokens.
    for source in &sources {
        lexer.feed(source);
    }

    let tokens = lexer.process(&ctx);

    
    // Feed the tokens into the parser.
    // We process all of the tokens into ASTs.
    for tokens in &tokens {
        parser.feed(tokens);
    }

    let asts = parser.process(&ctx);

    // We check for errors
    if !ctx.errors.borrow().is_empty() {
        println!("Errors encountered during compilation:");
        for error in ctx.errors.borrow().iter() {
            println!("{}", error);
        }
        return;
    }


    // Feed the ASTs into the analyzer and anaylze them
    for ast in asts {
        analyzer.feed(ast);
    }

    let asts = analyzer.process(&ctx);

    // We check for errors
    if !ctx.errors.borrow().is_empty() {
        println!("Errors encountered during compilation:");
        for error in ctx.errors.borrow().iter() {
            println!("{}", error);
        }
        return;
    }


    println!("Analyzed ASTs: {:#?}", asts);

}