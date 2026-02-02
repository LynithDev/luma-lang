use crate::ast::*;

use crate::stages::analyzer::{AnalyzerContext, AnalyzerStage};

pub struct ScopeIdentification;

impl AnalyzerStage for ScopeIdentification {
    fn name(&self) -> String {
        String::from("scope_identification")
    }

    fn analyze(&mut self, ctx: &AnalyzerContext, input: &mut Ast) {
        self.traverse(ctx, input);
    }
}

impl AstVisitor<'_> for ScopeIdentification {
    type Ctx = AnalyzerContext;

    fn enter_scope(&mut self, ctx: &Self::Ctx) {
        ctx.scopes.borrow_mut().enter_scope();
    }

    fn exit_scope(&mut self, ctx: &Self::Ctx) {
        ctx.scopes.borrow_mut().exit_scope();
    }

    fn visit_stmt(&mut self, ctx: &Self::Ctx, stmt: &mut Stmt) {
        stmt.scope_id = Some(ctx.scopes.borrow().current_scope());
    }

    fn visit_expr(&mut self, ctx: &Self::Ctx, expr: &mut Expr) {
        expr.scope_id = Some(ctx.scopes.borrow().current_scope());
    }
}