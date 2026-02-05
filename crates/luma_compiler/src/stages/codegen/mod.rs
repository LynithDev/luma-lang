use crate::{CompilerContext, CompilerStage, aast::*, bytecode::Bytecode};
use luma_diagnostic::CompilerResult;

pub use generator::BytecodeGen;

pub mod error;
pub mod chunks;
pub(super) mod ctx;
mod generator;

pub struct CodegenStage;

impl CodegenStage {
    pub fn new() -> Self {
        Self
    }
}

impl CompilerStage<'_> for CodegenStage {
    type Input = Vec<AnnotatedAst>;

    type Output = CompilerResult<Vec<Bytecode>>;

    fn name() -> &'static str {
        "codegen"
    }

    fn process(self, _ctx: &CompilerContext, input: Self::Input) -> Self::Output {
        let mut bytecodes = Vec::new();

        for ast in input {
            let bytecode = BytecodeGen::generate(ast)?;
        
            bytecodes.push(bytecode);
        }

        Ok(bytecodes)
    }
}