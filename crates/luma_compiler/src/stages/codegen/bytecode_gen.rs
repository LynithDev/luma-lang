use luma_diagnostic::CompilerResult;

use crate::{aast::AnnotatedAst, bytecode::ModuleBytecode, stages::codegen::chunk::{ChunkBuilder, TopLevelChunk}};

pub struct BytecodeGen {
    
}

impl BytecodeGen {
    pub fn generate(mut ast: AnnotatedAst) -> CompilerResult<ModuleBytecode> {
        let chunk = ChunkBuilder.build(&mut ast)?;

        Ok(ModuleBytecode {
            source_id: ast.span.source_id,
            chunk: TopLevelChunk {
                code: chunk,
                functions: vec![],
            },
        })
    }
}