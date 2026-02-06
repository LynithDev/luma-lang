use crate::ast::*;

use crate::{
    CompilerContext, CompilerStage, stages::{lexer::Token, parser::parse::TokenParser}
};

pub mod error;

mod ctx;
mod parse;
mod parse_expr;
mod parse_stmt;
mod parse_util;

#[cfg(test)]
mod tests;

pub struct ParserStage;

impl<'stage> CompilerStage<'stage> for ParserStage {
    type Input = &'stage [Vec<Token>];

    type Output = Vec<Ast>;

    fn name() -> &'static str {
        "parser"
    }
    
    fn process(self, ctx: &CompilerContext, input: Self::Input) -> Self::Output {
        let mut errors = ctx.get_errors_mut();
        
        input.iter()
            .map(|tokens| {
                TokenParser::new(tokens).parse_tokens(&mut errors)
            })
            .collect()
    }
}