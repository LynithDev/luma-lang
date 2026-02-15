use crate::{
    TypeKind,
    ast::*,
    stages::analyzer::{
        AnalyzerContext, AnalyzerPass, passes::_01_ast::TypeInference, type_cache::TypeCacheEntry,
    },
};

pub struct TypeSolving;

impl AnalyzerPass<Ast> for TypeSolving {
    fn name(&self) -> String {
        String::from("type_solving")
    }

    fn analyze(&self, ctx: &mut AnalyzerContext, input: &mut Ast) {
        for stmt in &mut input.statements {
            self.infer_stmt(ctx, &TypeCacheEntry::Concrete(TypeKind::Unit), stmt);
        }
    }

    fn continue_after_error(&self) -> bool {
        false
    }
}

#[allow(unused)]
impl TypeSolving {
    fn infer_stmt(
        &self,
        ctx: &mut AnalyzerContext,
        contextual_type: &TypeCacheEntry,
        stmt: &mut Stmt,
    ) {
        match &mut stmt.item {
            StmtKind::Expr(expr) => {
                self.infer_expr(ctx, contextual_type, expr);
            }
            StmtKind::Func(func_decl) => {
                let symbol_id = func_decl.symbol.unwrap_id();

                let type_entry = {
                    let ty_cache = ctx.type_cache.borrow();
                    ty_cache.get(symbol_id).cloned().unwrap()
                };

                let body_type = self.infer_expr(ctx, &type_entry, &mut func_decl.body);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&type_entry, &body_type) {
                    ctx.diagnostic(err.span(func_decl.symbol.span));
                }
            }
            StmtKind::Return(return_stmt) => todo!(),
            StmtKind::Struct(struct_decl) => todo!(),
            StmtKind::Var(var_decl) => {
                let symbol_id = var_decl.symbol.unwrap_id();

                let type_entry = {
                    let ty_cache = ctx.type_cache.borrow();
                    ty_cache.get(symbol_id).cloned().unwrap()
                };

                let init_type = self.infer_expr(ctx, &type_entry, &mut var_decl.initializer);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&type_entry, &init_type) {
                    ctx.diagnostic(err.span(var_decl.symbol.span));
                }
            }
        }
    }

    fn infer_expr(
        &self,
        ctx: &mut AnalyzerContext,
        contextual_type: &TypeCacheEntry,
        expr: &mut Expr,
    ) -> TypeCacheEntry {
        match &mut expr.item {
            ExprKind::Assign(assign_expr) => todo!(),
            ExprKind::Binary(binary_expr) => {
                let left_type = self.infer_expr(ctx, contextual_type, &mut binary_expr.left);
                let right_type = self.infer_expr(ctx, contextual_type, &mut binary_expr.right);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&left_type, &right_type) {
                    ctx.diagnostic(err.span(binary_expr.operator.span));
                }

                left_type
            }
            ExprKind::Block(block_expr) => {
                for stmt in &mut block_expr.statements {
                    self.infer_stmt(ctx, contextual_type, stmt);
                }

                if let Some(expr) = &mut block_expr.tail_expr {
                    self.infer_expr(ctx, contextual_type, expr)
                } else {
                    TypeCacheEntry::Concrete(TypeKind::Unit)
                }
            }
            ExprKind::Call(call_expr) => todo!(),
            ExprKind::Get(get_expr) => todo!(),
            ExprKind::Group(expr) => self.infer_expr(ctx, contextual_type, expr),
            ExprKind::Ident(ident_expr) => {
                let symbol_id = ident_expr.symbol.unwrap_id();
                ctx.type_cache
                    .borrow()
                    .get(symbol_id)
                    .cloned()
                    .unwrap_or_else(|| {
                        let id = ctx.type_cache.borrow_mut().insert_relative(symbol_id);

                        TypeCacheEntry::Relative(id)
                    })
            }
            ExprKind::If(if_expr) => {
                let cond_type = self.infer_expr(
                    ctx,
                    &TypeCacheEntry::Concrete(TypeKind::Bool),
                    &mut if_expr.condition,
                );

                if let Err(err) = ctx
                    .type_cache
                    .borrow_mut()
                    .unify(&cond_type, &TypeCacheEntry::Concrete(TypeKind::Bool))
                {
                    ctx.diagnostic(err.span(if_expr.condition.span));
                    return TypeCacheEntry::Concrete(TypeKind::Error);
                }

                let then_type = self.infer_expr(ctx, contextual_type, &mut if_expr.then_branch);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(contextual_type, &then_type) {
                    ctx.diagnostic(err.span(if_expr.then_branch.span));
                    return TypeCacheEntry::Concrete(TypeKind::Error);
                }

                let else_type = if let Some(else_branch) = &mut if_expr.else_branch {
                    let entry = self.infer_expr(ctx, contextual_type, else_branch);
                    if let Err(err) = ctx.type_cache.borrow_mut().unify(contextual_type, &entry) {
                        ctx.diagnostic(err.span(else_branch.span));
                        return TypeCacheEntry::Concrete(TypeKind::Error);
                    }
                    entry
                } else {
                    TypeCacheEntry::Concrete(TypeKind::Unit)
                };

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&then_type, &else_type) {
                    ctx.diagnostic(err.span(expr.span));
                    return TypeCacheEntry::Concrete(TypeKind::Error);
                }

                then_type
            }
            ExprKind::Literal(literal_expr) => {
                if let TypeCacheEntry::Relative(id) = contextual_type {
                    if let Some(resolved) = ctx.type_cache.borrow_mut().resolve(contextual_type) {
                        return TypeCacheEntry::Concrete(resolved);
                    }

                    return contextual_type.clone();
                }

                TypeInference::infer_literal_type(ctx, contextual_type, literal_expr, expr.span)
            }
            ExprKind::Struct(struct_expr) => todo!(),
            ExprKind::TupleLiteral(tuple_expr) => todo!(),
            ExprKind::Unary(unary_expr) => todo!(),
        }
    }
}
