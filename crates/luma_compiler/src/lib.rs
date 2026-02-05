#![allow(clippy::new_without_default)]
#![feature(iterator_try_collect)]


mod ctx;
use std::cell::{Ref, RefCell};

pub use ctx::CompilerContext;

pub mod stages;
use luma_core::CodeSource;
use luma_diagnostic::LumaError;
pub use stages::{analyzer::AnalyzerStage, codegen::CodegenStage, lexer::LexerStage, parser::ParserStage};

mod representation;
pub use representation::*;

use crate::{representation::{aast::AnnotatedAst, ast::Ast}, stages::lowering::AstLoweringStage};

pub trait CompilerStage<'stage> {
    type Input;
    type Output;

    fn name() -> &'static str;
    
    fn process(self, ctx: &CompilerContext, input: Self::Input) -> Self::Output;
}

pub struct LumaCompiler {
    current_stage_name: RefCell<String>,
    ctx: CompilerContext,
}

impl LumaCompiler {
    pub fn new() -> Self {
        Self {
            current_stage_name: RefCell::new(String::new()),
            ctx: CompilerContext::new(),
        }
    }

    pub fn compile(&self, sources: &[CodeSource]) -> bool {
        self.run_pipeline(sources).is_ok()
    }

    pub fn current_stage(&self) -> Ref<'_, String> {
        self.current_stage_name.borrow()
    }

    pub fn errors(&self) -> Ref<'_, Vec<LumaError>> {
        self.ctx.errors.borrow()
    }

    fn run_pipeline(&self, sources: &[CodeSource]) -> Result<(), ()> {
        let tokens = self.run_stage(LexerStage, sources)?;
        let asts = self.run_stage(ParserStage, &tokens)?;
        let asts = self.run_stage(AnalyzerStage::<Ast>::default(), asts)?;
        let aasts = self.run_stage(AstLoweringStage, asts)?;
        let aasts = self.run_stage(AnalyzerStage::<AnnotatedAst>::default(), aasts)?;
        let bytecodes = self.run_stage(CodegenStage, aasts)?;

        dbg!(bytecodes);

        Ok(())
    }

    fn run_stage<'stage, S>(
        &self,
        stage: S,
        input: S::Input,
    ) -> Result<S::Output, ()>
    where 
        S: CompilerStage<'stage>,
    {
        *self.current_stage_name.borrow_mut() = String::from(S::name());
        let output = stage.process(&self.ctx, input);

        let errors = self.ctx.errors.borrow();
        if errors.is_empty() {
            return Ok(output);
        }

        Err(())
    }
}
