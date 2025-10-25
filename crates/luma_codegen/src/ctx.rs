use luma_core::bytecode::prelude::*;
use luma_diagnostic::{DiagnosticResult, Reporter};
use luma_semantics::{ParsedCodeKind, ParsedCodeSource};

use crate::codegen::ChunkBuilder;

#[derive(Debug)]
pub struct CodegenContext<'a> {
    pub source: &'a ParsedCodeSource,
    pub reporter: Reporter,
}

impl<'a> CodegenContext<'a> {
    pub fn new(reporter: &Reporter, source: &'a ParsedCodeSource) -> Self {
        Self { 
            source,
            reporter: reporter.clone(),
        }
    }
}

impl CodegenContext<'_> {
    pub fn generate(&mut self) -> DiagnosticResult<()> {
        let mut bytecode = Bytecode {
            functions: Vec::new(),
            top_level: Chunk::new(),
        };
        
        for statement in self.source.code.borrow().as_hir_unchecked().statements.iter() {
            let chunk = &mut bytecode.top_level;
            let mut builder = ChunkBuilder::new(chunk);

            builder.gen_statement(statement)?;
        }

        self.source.code.replace(ParsedCodeKind::Bytecode(bytecode));

        Ok(())
    }

    
}