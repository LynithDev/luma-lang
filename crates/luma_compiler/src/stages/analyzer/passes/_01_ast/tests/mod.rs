use crate::ast::*;
use luma_core::{CodeSource, Span};

use crate::{AnalyzerStage, CompilerContext, CompilerStage, LexerStage, ParserStage};

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
    let ctx = CompilerContext::new();

    let lexer = LexerStage;
    let parser = ParserStage;
    let analyzer = AnalyzerStage::default();

    let binding = CodeSource::from(src);
    let mut tokens = lexer.process(&ctx, &[binding]);

    for tokens in &mut tokens {
        for token in &mut *tokens {
            token.span = Span::default();
        }
    }

    let asts = parser.process(&ctx, &tokens);

    if !ctx.errors.borrow().is_empty() {
        println!("Errors encountered during compilation:");
        for error in ctx.errors.borrow().iter() {
            println!("{}", error);
        }

        return None;
    }

    let asts = analyzer.process(&ctx, asts);

    if !ctx.errors.borrow().is_empty() {
        println!("Errors encountered during compilation:");
        for error in ctx.errors.borrow().iter() {
            println!("{}", error);
        }

        return None;
    }

    if asts.is_empty() {
        None
    } else {
        Some(asts.into_iter().next().unwrap())
    }
}