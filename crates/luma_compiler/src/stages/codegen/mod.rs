use crate::{
    CompilerContext, CompilerStage, aast::*, bytecode::ModuleBytecode,
    stages::codegen::module::BytecodeGen,
};

pub mod chunk;
mod diagnostics;
pub mod module;
pub mod scope;
pub mod stores;

pub use diagnostics::*;

pub struct CodegenStage;

impl CodegenStage {
    pub fn new() -> Self {
        Self
    }
}

impl CompilerStage<'_> for CodegenStage {
    type Input = Vec<AnnotatedAst>;

    type Output = Vec<ModuleBytecode>;

    fn name() -> &'static str {
        "codegen"
    }

    fn process(self, ctx: &CompilerContext, input: Self::Input) -> Self::Output {
        let mut bytecodes = Vec::new();

        for ast in input {
            let bytecode = match BytecodeGen::generate(ast) {
                Ok(bc) => bc,
                Err(err) => {
                    ctx.add_diag(err);
                    return Vec::new();
                }
            };

            bytecodes.push(bytecode);
        }

        bytecodes
    }
}
