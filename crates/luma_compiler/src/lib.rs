#![allow(clippy::new_without_default)]
#![feature(iterator_try_collect)]


mod ctx;
pub use ctx::CompilerContext;

pub mod stages;
pub use stages::{analyzer::AnalyzerStage, codegen::CodegenStage, lexer::LexerStage, parser::ParserStage};

mod representation;
pub use representation::*;

pub trait CompilerStage<'stage> {
    type Input;
    type Output;

    fn name() -> &'static str;
    
    fn process(self, ctx: &CompilerContext, input: Self::Input) -> Self::Output;
}

