use crate::{ScopeId, ast::*};

use crate::stages::analyzer::{AnalyzerContext, AnalyzerPass};

pub struct ScopeIdentification;

impl AnalyzerPass<Ast> for ScopeIdentification {
    fn name(&self) -> String {
        String::from("scope_identification")
    }

    fn analyze(&self, ctx: &mut AnalyzerContext, input: &mut Ast) {
        self.traverse(ctx, input);
    }

    fn continue_after_error(&self) -> bool {
        false
    }
}

impl AstVisitor<'_> for ScopeIdentification {
    type Ctx = AnalyzerContext;

    fn enter_scope(&self, ctx: &mut Self::Ctx, _entering_scope_id: Option<ScopeId>) {
        ctx.scopes.borrow_mut().enter_scope();
    }

    fn exit_scope(&self, ctx: &mut Self::Ctx, _leaving_scope_id: Option<ScopeId>) {
        ctx.scopes.borrow_mut().exit_scope();
    }

    fn visit_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {
        stmt.scope_id = Some(ctx.scopes.borrow().current_scope());
    }

    fn visit_expr(&self, ctx: &mut Self::Ctx, expr: &mut Expr) {
        expr.scope_id = Some(ctx.scopes.borrow().current_scope());
    }

    fn visit_func_param<'node>(&self, ctx: &mut Self::Ctx, _func: &'node FuncDeclStmt, param: &'node mut FuncParam) {
        param.scope_id = Some(ctx.scopes.borrow().current_scope());
    }
}