use luma_core::ast::*;
use luma_diagnostic::LumaError;

use crate::{
    CompilerContext, CompilerStage, lexer::Token, parser::{parse::TokenParser, error::ParserErrorKind}
};

pub mod error;

mod ctx;
mod parse;
mod parse_expr;
mod parse_stmt;
mod parse_util;

#[cfg(test)]
mod tests;

pub struct Parser<'tokens> {
    parsers: Vec<TokenParser<'tokens>>,
}

impl Parser<'_> {
    pub fn new<'tokens>() -> Parser<'tokens> {
        Parser {
            parsers: Vec::new(),
        }
    }

    pub fn parse(tokens: &[Token]) -> (Ast, Vec<LumaError>) {
        let mut errors = Vec::new();
        
        let parser = TokenParser::new(tokens);
        let ast = parser.parse_tokens(&mut errors);
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
        self.parsers.push(TokenParser::new(input));
    }
    
    fn process(self, ctx: &CompilerContext) -> Self::ProcessedOutput {
        let mut errors = ctx.errors.borrow_mut();
        
        self.parsers.into_iter()
            .map(|ctx| {
                ctx.parse_tokens(&mut errors)
            })
            .collect()
    }
}