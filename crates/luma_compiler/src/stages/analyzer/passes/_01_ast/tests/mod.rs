use crate::{CompilerOptions, ast::*, compiler::run_stage, stages::lexer::LexerOptions};
use luma_core::{CodeSource, CodeSourceId};

use crate::{AnalyzerStage, CompilerContext, LexerStage, ParserStage};

pub mod _03_type_inference;

mod macros {
    macro_rules! extract_stmt {
        ($pat:pat = $ast:tt[$idx:expr]) => {
            let $pat = $ast.statements[$idx].item.clone()
            else {
                panic!("expected different statement kind");
            };
        };
    }

    

    pub(crate) use extract_stmt;
}

pub(crate) use macros::*;

pub fn analyze_source(src: &str) -> Option<Ast> {
    let source = CodeSource::from(src);
    
    let mut ctx = CompilerContext::configure(CompilerOptions {
        lexer: LexerOptions {
            zeroed_spans: true,
            ..Default::default()
        },
        ..Default::default()
    });
    
    ctx.sources.add_source(source);

    let tokens = run_stage(&ctx, LexerStage, vec![CodeSourceId::new(0)]).ok()?;
    let asts = run_stage(&ctx, ParserStage, &tokens).ok()?;
    let asts = run_stage(&ctx, AnalyzerStage::<Ast>::default(), asts).ok()?;

    if asts.is_empty() {
        None
    } else {
        Some(asts.into_iter().next().unwrap())
    }
}