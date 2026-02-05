use crate::ast::*;
use luma_diagnostic::LumaError;

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

impl ParserStage {
    pub fn parse(tokens: &[Token]) -> (Ast, Vec<LumaError>) {
        let mut errors = Vec::new();
        
        let parser = TokenParser::new(tokens);
        let ast = parser.parse_tokens(&mut errors);
        (ast, errors)
    }
}

impl<'stage> CompilerStage<'stage> for ParserStage {
    type Input = &'stage [Vec<Token>];

    type Output = Vec<Ast>;

    fn name() -> &'static str {
        "parser"
    }
    
    fn process(self, ctx: &CompilerContext, input: Self::Input) -> Self::Output {
        let mut errors = ctx.errors.borrow_mut();
        
        input.iter()
            .map(|tokens| {
                TokenParser::new(tokens).parse_tokens(&mut errors)
            })
            .collect()
    }
}