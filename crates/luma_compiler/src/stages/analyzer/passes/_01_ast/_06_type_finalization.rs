use luma_diagnostic::error;

use crate::{
    Type, TypeKind,
    ast::*,
    stages::analyzer::{
        AnalyzerContext, AnalyzerError, AnalyzerPass, passes::_01_ast::TypeInference, type_cache::TypeCacheEntry,
    },
};

pub struct TypeFinalization;

impl AnalyzerPass<Ast> for TypeFinalization {
    fn name(&self) -> String {
        String::from("type_finalization")
    }

    fn analyze(&self, ctx: &mut AnalyzerContext, input: &mut Ast) {
        for stmt in input.statements.iter_mut() {
            self.finalize_stmt(ctx, &TypeCacheEntry::Concrete(TypeKind::Unit), stmt);
        }
    }

    fn continue_after_error(&self) -> bool {
        false
    }
}

#[allow(unused)]
impl TypeFinalization {
    fn finalize_stmt(
        &self,
        ctx: &mut AnalyzerContext,
        contextual_type: &TypeCacheEntry,
        stmt: &mut Stmt,
    ) {
        match &mut stmt.item {
            StmtKind::Expr(expr) => {
                self.finalize_expr(ctx, contextual_type, expr);
            }
            StmtKind::Func(func_decl) => {
                let symbol_id = func_decl.symbol.unwrap_id();

                let type_entry = {
                    let ty_cache = ctx.type_cache.borrow();
                    ty_cache.get(symbol_id).cloned().unwrap()
                };

                let resolved_ty = ctx.type_cache.borrow_mut().resolve(&type_entry).unwrap();
                func_decl.return_type = Some(Type::unspanned(resolved_ty.clone()));

                self.finalize_expr(ctx, &type_entry, &mut func_decl.body);
            }
            StmtKind::Return(return_stmt) => todo!(),
            StmtKind::Struct(struct_decl) => todo!(),
            StmtKind::Var(var_decl) => {
                let symbol_id = var_decl.symbol.unwrap_id();

                let type_entry = {
                    let ty_cache = ctx.type_cache.borrow();
                    ty_cache.get(symbol_id).cloned().unwrap()
                };

                let resolved_ty = ctx.type_cache.borrow_mut().resolve(&type_entry).unwrap();
                var_decl.ty = Some(Type::unspanned(resolved_ty.clone()));

                self.finalize_expr(ctx, &type_entry, &mut var_decl.initializer);
            }
        }
    }

    fn finalize_expr(
        &self,
        ctx: &mut AnalyzerContext,
        contextual_type: &TypeCacheEntry,
        expr: &mut Expr,
    ) {
        match self.infer_expr(ctx, contextual_type, expr) {
            Some(resolved_ty) => {
                expr.set_type(resolved_ty);
            }
            None => {
                ctx.diagnostic(error!(AnalyzerError::TypeInferenceFailure).span(expr.span));
            }
        }
    }

    fn infer_expr(
        &self,
        ctx: &mut AnalyzerContext,
        contextual_type: &TypeCacheEntry,
        expr: &mut Expr,
    ) -> Option<TypeKind> {
        match &mut expr.item {
            ExprKind::Assign(assign_expr) => todo!(),
            ExprKind::Binary(binary_expr) => {
                self.finalize_expr(ctx, contextual_type, &mut binary_expr.left);
                self.finalize_expr(ctx, contextual_type, &mut binary_expr.right);

                binary_expr.left.ty.clone()
            }
            ExprKind::Block(block_expr) => {
                for stmt in &mut block_expr.statements {
                    self.finalize_stmt(ctx, contextual_type, stmt);
                }

                if let Some(expr) = &mut block_expr.tail_expr {
                    self.finalize_expr(ctx, contextual_type, expr);
                    expr.ty.clone()
                } else {
                    Some(TypeKind::Unit)
                }
            }
            ExprKind::Call(call_expr) => todo!(),
            ExprKind::Get(get_expr) => todo!(),
            ExprKind::Group(expr) => {
                self.finalize_expr(ctx, contextual_type, expr);
                expr.ty.clone()
            }
            ExprKind::Ident(ident_expr) => {
                let symbol_id = ident_expr.symbol.unwrap_id();

                let mut ty_cache = ctx.type_cache.borrow_mut();

                if let Some(type_entry) = ty_cache.get(symbol_id).cloned() {
                    ty_cache.resolve(&type_entry)
                } else {
                    None
                }
            }
            ExprKind::If(if_expr) => todo!(),
            ExprKind::Literal(literal_expr) => {
                TypeInference::infer_literal_type(ctx, contextual_type, literal_expr).as_concrete().cloned()
            }
            ExprKind::Struct(struct_expr) => todo!(),
            ExprKind::TupleLiteral(tuple_expr) => todo!(),
            ExprKind::Unary(unary_expr) => todo!(),
        }
    }
}
