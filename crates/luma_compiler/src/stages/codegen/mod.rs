use crate::{CompilerContext, ast::*};

use crate::{CompilerStage, stages::codegen::error::CodegenErrorKind};

pub mod error;

pub struct Codegen {
    asts: Vec<Ast>,
}

impl Codegen {
    pub fn new() -> Self {
        Self { 
            asts: Vec::new() 
        }
    }
}

impl CompilerStage for Codegen {
    type Input = Ast;

    type ProcessedOutput = ();

    type ErrorKind = CodegenErrorKind;

    fn name() -> String {
        String::from("codegen")
    }

    fn feed(&mut self, input: Self::Input) {
        self.asts.push(input);
    }

    fn process(self, ctx: &CompilerContext) -> Self::ProcessedOutput {
        
        

    }
}

impl AstVisitor<'_> for Codegen {
    type Ctx = CompilerContext;

    fn visit_stmt(&mut self, ctx: &Self::Ctx, stmt: &mut Stmt) {
        
    }
}