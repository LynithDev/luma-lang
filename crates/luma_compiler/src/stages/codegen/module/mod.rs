use luma_diagnostic::CompilerResult;

use crate::{aast::AnnotatedAst, bytecode::ModuleBytecode, stages::codegen::chunk::{ChunkBuilder, TopLevelChunk}};

mod ctx;

pub use ctx::*;

pub struct BytecodeGen {
    
}

impl BytecodeGen {
    pub fn generate(mut ast: AnnotatedAst) -> CompilerResult<ModuleBytecode> {
        let ctx = ModuleContext::new();
        let chunk = ChunkBuilder.build(&ctx, &mut ast.statements)?;

        Ok(ModuleBytecode {
            source_id: ast.span.source_id,
            chunk: TopLevelChunk {
                code: chunk,
                functions: vec![],
            },
        })
    }
}