use crate::ast::*;

use crate::stages::analyzer::{AnalyzerContext, AnalyzerPass};

pub struct ScopeIdentification;

impl AnalyzerPass for ScopeIdentification {
    fn name(&self) -> String {
        String::from("scope_identification")
    }

    fn analyze(&mut self, ctx: &mut AnalyzerContext, input: &mut Ast) {
        self.traverse(ctx, input);
    }
}

impl AstVisitor<'_> for ScopeIdentification {
    type Ctx = AnalyzerContext;

    fn enter_scope(&mut self, ctx: &mut Self::Ctx) {
        ctx.scopes.borrow_mut().enter_scope();
    }

    fn exit_scope(&mut self, ctx: &mut Self::Ctx) {
        ctx.scopes.borrow_mut().exit_scope();
    }

    fn visit_stmt(&mut self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {
        stmt.scope_id = Some(ctx.scopes.borrow().current_scope());
    }

    fn visit_expr(&mut self, ctx: &mut Self::Ctx, expr: &mut Expr) {
        expr.scope_id = Some(ctx.scopes.borrow().current_scope());
    }
}