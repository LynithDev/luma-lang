use crate::{TypeKind, ast::*, stages::analyzer::{AnalyzerContext, AnalyzerPass, passes::_01_ast::TypeInference, type_cache::{TypeCache, TypeCacheEntry}}};

pub struct TypeSolving;

impl AnalyzerPass<Ast> for TypeSolving {
    fn name(&self) -> String {
        String::from("type_solving")
    }

    fn analyze(&self, ctx: &mut AnalyzerContext, input: &mut Ast) {
        self.traverse(ctx, input);
    }
}

impl AstVisitor<'_> for TypeSolving {
    type Ctx = AnalyzerContext;

    fn visit_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {
        match &stmt.item {
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

                let init_type_entry = Self::infer_expr(ctx, var_decl.ty.as_ref().map(|ty| &ty.kind), &var_decl.initializer);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&type_entry, &init_type_entry) {
                    ctx.diagnostic(err.span(var_decl.symbol.span));
                }
            },
        }
    }
}

impl TypeSolving {
    fn infer_expr(ctx: &AnalyzerContext, ty_ctx: Option<&TypeKind>, expr: &Expr) -> TypeCacheEntry {
        match &expr.item {
            ExprKind::Assign(assign_expr) => todo!(),
            ExprKind::Binary(binary_expr) => {
                let left_type = Self::infer_expr(ctx, None, &binary_expr.left);
                let right_type = Self::infer_expr(ctx, None, &binary_expr.right);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&left_type, &right_type) {
                    ctx.diagnostic(err.span(binary_expr.operator.span));
                }

                left_type
            },
            ExprKind::Block(block_expr) => todo!(),
            ExprKind::Call(call_expr) => todo!(),
            ExprKind::Get(get_expr) => todo!(),
            ExprKind::Group(expr) => todo!(),
            ExprKind::Ident(ident_expr) => {
                let symbol_id = ident_expr.symbol.unwrap_id();
                ctx.type_cache.borrow().get(symbol_id)
                    .cloned()
                    .unwrap_or_else(|| {
                        let id = ctx.type_cache.borrow_mut().insert_relative(symbol_id);

                        TypeCacheEntry::Relative(id)
                    })
            },
            ExprKind::If(if_expr) => todo!(),
            ExprKind::Literal(literal_expr) => {
                let ty = TypeInference::infer_literal_type(ty_ctx, literal_expr)
                    .unwrap_or(TypeKind::Unit);

                TypeCacheEntry::Concrete(ty)
            },
            ExprKind::Struct(struct_expr) => todo!(),
            ExprKind::TupleLiteral(tuple_expr) => todo!(),
            ExprKind::Unary(unary_expr) => todo!(),
        }
    }
}