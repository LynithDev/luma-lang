#![allow(clippy::new_without_default)]
#![feature(iterator_try_collect)]
#![feature(try_blocks)]

mod compiler;
mod ctx;
pub mod diagnostics;
mod options;
mod representation;
pub mod stages;

pub use compiler::LumaCompiler;
pub use ctx::CompilerContext;
pub use options::CompilerOptions;
pub use representation::*;
pub use stages::{
    analyzer::AnalyzerStage, codegen::CodegenStage, lexer::LexerStage, lowering::AstLoweringStage,
    parser::ParserStage,
};

pub trait CompilerStage<'stage> {
    type Input;
    type Output;

    fn name() -> &'static str;

    fn process(self, ctx: &CompilerContext, input: Self::Input) -> Self::Output;
}
