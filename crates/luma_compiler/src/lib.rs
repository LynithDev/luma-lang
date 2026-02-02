#![allow(clippy::new_without_default)]

mod ctx;
pub use ctx::CompilerContext;

pub mod stages;
pub use stages::{analyzer::Analyzer, codegen::Codegen, lexer::Lexer, parser::Parser};

mod representation;
pub use representation::*;

pub trait CompilerStage {
    type Input;
    type ProcessedOutput;
    type ErrorKind;

    fn name() -> String;
    
    fn feed(&mut self, input: Self::Input);
    fn process(self, ctx: &CompilerContext) -> Self::ProcessedOutput;
}

