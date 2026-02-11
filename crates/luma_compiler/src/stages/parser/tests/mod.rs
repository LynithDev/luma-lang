use crate::{
    CompilerContext, CompilerOptions, LexerStage, ParserStage, ast::Ast, compiler::run_stage, stages::lexer::LexerOptions,
};

pub mod parse_func;
pub mod parse_var;

pub fn parse_ast(src: &str) -> Ast {
    let mut ctx = CompilerContext::configure(CompilerOptions {
        lexer: LexerOptions {
            zeroed_spans: true,
            ..Default::default()
        },
        ..Default::default()
    });
    
    ctx.sources.add_source(src.into());

    let tokens = run_stage(&ctx, LexerStage, vec![0.into()]).expect("lexing failed");
    let asts = run_stage(&ctx, ParserStage, &tokens).expect("parsing failed");

    asts.into_iter().next().expect("expected at least one AST")
}
