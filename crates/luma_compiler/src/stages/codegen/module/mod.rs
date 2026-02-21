use luma_diagnostic::CompilerResult;

use crate::{aast::AnnotatedAst, bytecode::ModuleBytecode, stages::codegen::chunk::{ChunkBuilder, FunctionChunk}};

mod ctx;

pub use ctx::*;

pub struct ModuleBuilder {
    
}

impl ModuleBuilder {
    pub fn generate(mut ast: AnnotatedAst) -> CompilerResult<ModuleBytecode> {
        let mut ctx = ModuleContext::new();
        
        // build top level chunk into a function chunk
        // the function chunk is a specially reserved function that serves as the "init" function for the module
        let top_level_chunk = ChunkBuilder.build_top_level(&mut ctx, &mut ast.statements)?;

        let init_func = FunctionChunk {
            code: top_level_chunk,
            arity: 0,
        };

        // collect all constants
        let constants = ctx.constant_table.constants;

        // collect all functions
        let mut functions = Vec::with_capacity(ctx.function_table.functions.len() + 1);
        functions.push(init_func);
        functions.append(&mut ctx.function_table.functions);

        Ok(ModuleBytecode {
            source_id: ast.span.source_id,
            constants,
            functions,
        })
    }
}