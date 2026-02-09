use luma_diagnostic::error;

use crate::{
    Type, TypeKind,
    ast::*,
    stages::analyzer::{
        AnalyzerContext, AnalyzerError, AnalyzerPass, passes::_01_ast::TypeInference,
    },
};

pub struct TypeFinalization;

impl AnalyzerPass<Ast> for TypeFinalization {
    fn name(&self) -> String {
        String::from("type_finalization")
    }

    fn analyze(&self, ctx: &mut AnalyzerContext, input: &mut Ast) {
        self.traverse(ctx, input);
    }
}

impl AstVisitor<'_> for TypeFinalization {
    type Ctx = AnalyzerContext;

    fn leave_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {
        self.finalize_stmt(ctx, None, stmt);
    }
}

impl TypeFinalization {
    fn finalize_stmt(
        &self,
        ctx: &mut AnalyzerContext,
        contextual_type: Option<&TypeKind>,
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

                self.finalize_expr(ctx, Some(&resolved_ty), &mut func_decl.body);
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

                self.finalize_expr(ctx, Some(&resolved_ty), &mut var_decl.initializer);
            }
        }
    }

    fn finalize_expr(
        &self,
        ctx: &mut AnalyzerContext,
        contextual_type: Option<&TypeKind>,
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
        contextual_type: Option<&TypeKind>,
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
                let mut last_type = None;

                for stmt in &mut block_expr.statements {
                    self.finalize_stmt(ctx, contextual_type, stmt);
                }

                if let Some(expr) = block_expr.return_value_mut() {
                    last_type = expr.ty.clone();
                }

                last_type.or(Some(TypeKind::Unit))
            }
            ExprKind::Call(call_expr) => todo!(),
            ExprKind::Get(get_expr) => todo!(),
            ExprKind::Group(expr) => {
                self.finalize_expr(ctx, contextual_type, expr);
                expr.ty.clone()
            }
            ExprKind::Ident(ident_expr) => {
                let symbol_id = ident_expr.symbol.unwrap_id();
                let type_entry = ctx.type_cache.borrow().get(symbol_id).cloned().unwrap();
                ctx.type_cache.borrow_mut().resolve(&type_entry)
            }
            ExprKind::If(if_expr) => todo!(),
            ExprKind::Literal(literal_expr) => {
                TypeInference::infer_literal_type(contextual_type, literal_expr)
            }
            ExprKind::Struct(struct_expr) => todo!(),
            ExprKind::TupleLiteral(tuple_expr) => todo!(),
            ExprKind::Unary(unary_expr) => todo!(),
        }
    }
}
