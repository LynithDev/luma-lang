use crate::{
    Type, TypeKind, ast::*, stages::analyzer::{AnalyzerContext, AnalyzerPass, passes::_01_ast::TypeInference}
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

    fn visit_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {
        match &mut stmt.item {
            StmtKind::Expr(expr) => todo!(),
            StmtKind::Func(func_decl) => todo!(),
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

                Self::finalize_expr(ctx, Some(&resolved_ty), &mut var_decl.initializer);
            }
        }
    }
}

impl TypeFinalization {
    fn finalize_expr(ctx: &AnalyzerContext, contextual_type: Option<&TypeKind>, expr: &mut Expr) {
        let ty = match &mut expr.item {
            ExprKind::Assign(assign_expr) => todo!(),
            ExprKind::Binary(binary_expr) => {
                Self::finalize_expr(ctx, contextual_type, &mut binary_expr.left);
                Self::finalize_expr(ctx, contextual_type, &mut binary_expr.right);

                binary_expr.left.ty.clone().unwrap()
            }
            ExprKind::Block(block_expr) => todo!(),
            ExprKind::Call(call_expr) => todo!(),
            ExprKind::Get(get_expr) => todo!(),
            ExprKind::Group(expr) => todo!(),
            ExprKind::Ident(ident_expr) => {
                let symbol_id = ident_expr.symbol.unwrap_id();
                let type_entry = ctx.type_cache.borrow().get(symbol_id).cloned().unwrap();
                ctx.type_cache.borrow_mut().resolve(&type_entry).unwrap()
            }
            ExprKind::If(if_expr) => todo!(),
            ExprKind::Literal(literal_expr) => {
                TypeInference::infer_literal_type(contextual_type, literal_expr).unwrap()
            }
            ExprKind::Struct(struct_expr) => todo!(),
            ExprKind::TupleLiteral(tuple_expr) => todo!(),
            ExprKind::Unary(unary_expr) => todo!(),
        };

        expr.set_type(ty);
    }
}
