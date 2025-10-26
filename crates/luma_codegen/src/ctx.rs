use luma_core::bytecode::prelude::*;
use luma_diagnostic::{DiagnosticReport, DiagnosticResult, Reporter};
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
        let mut bytecode = Bytecode::default();
        let mut builder = ChunkBuilder::new(&mut bytecode.top_level, &mut bytecode.functions);

        for statement in self.source.code.borrow().as_hir_unchecked().statements.iter() {
            if let Err(err) = builder.gen_statement(statement) {
                self.reporter.report(DiagnosticReport {
                    message: Box::new(err),
                    cursor: statement.cursor,
                    span: statement.span,
                });
            }
        }

        self.source.code.replace(ParsedCodeKind::Bytecode(bytecode));

        Ok(())
    }

    
}