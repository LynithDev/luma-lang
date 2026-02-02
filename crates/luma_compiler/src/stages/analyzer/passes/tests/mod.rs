use crate::ast::*;
use luma_core::{CodeSource, Span};

use crate::{Analyzer, CompilerContext, CompilerStage, Lexer, Parser};

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

    let mut lexer = Lexer::new();
    let mut parser = Parser::new();
    let mut analyzer = Analyzer::default();

    let binding = CodeSource::from(src);
    lexer.feed(&binding);

    let mut tokens = lexer.process(&ctx);

    for tokens in &mut tokens {
        for token in &mut *tokens {
            token.span = Span::default();
        }

        parser.feed(tokens);
    }

    let asts = parser.process(&ctx);

    if !ctx.errors.borrow().is_empty() {
        println!("Errors encountered during compilation:");
        for error in ctx.errors.borrow().iter() {
            println!("{}", error);
        }

        return None;
    }

    for ast in asts {
        analyzer.feed(ast);
    }

    let asts = analyzer.process(&ctx);

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