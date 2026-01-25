use luma_core::ast::*;
use luma_diagnostic::LumaError;

use crate::{
    CompilerContext, CompilerStage, lexer::Token, parser::{context::ParserContext, error::ParserErrorKind}
};

pub mod error;

mod context;
mod parse_expr;
mod parse_stmt;
mod parse_util;

#[cfg(test)]
mod tests;

pub struct Parser<'tokens> {
    contexts: Vec<ParserContext<'tokens>>,
}

impl Parser<'_> {
    pub fn new<'tokens>() -> Parser<'tokens> {
        Parser {
            contexts: Vec::new(),
        }
    }

    pub fn parse(tokens: &[Token]) -> (Ast, Vec<LumaError>) {
        let mut errors = Vec::new();
        
        let context = ParserContext::new(tokens);
        let ast = context.parse_tokens(&mut errors);
        (ast, errors)
    }
}

impl<'tokens> CompilerStage for Parser<'tokens> {
    type Input = &'tokens [Token];

    type ProcessedOutput = Vec<Ast>;

    type ErrorKind = ParserErrorKind;

    fn name() -> String {
        String::from("parser")
    }

    fn feed(&mut self, input: Self::Input) {
        self.contexts.push(ParserContext::new(input));
    }
    
    fn process(self, ctx: &CompilerContext) -> Self::ProcessedOutput {
        let mut errors = ctx.errors.borrow_mut();
        
        self.contexts.into_iter()
            .map(|ctx| {
                ctx.parse_tokens(&mut errors)
            })
            .collect()
    }
}