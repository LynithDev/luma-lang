use crate::{CompilerContext, CompilerStage, aast::*, bytecode::ModuleBytecode, stages::codegen::bytecode_gen::BytecodeGen};

pub mod error;
pub mod chunk;
mod bytecode_gen;

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
                    ctx.errors.borrow_mut().push(err);
                    return Vec::new();
                }
            };
        
            bytecodes.push(bytecode);
        }

        bytecodes
    }
}