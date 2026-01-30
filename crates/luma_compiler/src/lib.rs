#![allow(clippy::new_without_default)]

mod ctx;
pub mod analyzer;
pub mod codegen;
pub mod lexer;
pub mod parser;

pub use analyzer::Analyzer;
pub use codegen::Codegen;
pub use ctx::CompilerContext;
pub use lexer::Lexer;
pub use parser::Parser;

pub trait CompilerStage {
    type Input;
    type ProcessedOutput;
    type ErrorKind;

    fn name() -> String;
    
    fn feed(&mut self, input: Self::Input);
    fn process(self, ctx: &CompilerContext) -> Self::ProcessedOutput;
}

