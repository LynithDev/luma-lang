use crate::{TypeKind, ast::*, stages::analyzer::{AnalyzerContext, AnalyzerPass, passes::_01_ast::TypeInference, type_cache::TypeCacheEntry}};

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

    fn leave_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {
        self.infer_stmt(ctx, None, stmt);
    }
}

#[allow(unused)]
impl TypeSolving {
    fn infer_stmt(&self, ctx: &mut AnalyzerContext, contextual_type: Option<&TypeKind>, stmt: &mut Stmt) {
        match &mut stmt.item {
            StmtKind::Expr(expr) => {
                self.infer_expr(ctx, contextual_type, expr);
            },
            StmtKind::Func(func_decl) => {
                let symbol_id = func_decl.symbol.unwrap_id();
                
                let type_entry = {
                    let ty_cache = ctx.type_cache.borrow();
                    ty_cache.get(symbol_id).cloned().unwrap()
                };

                let body_type = self.infer_expr(ctx, type_entry.as_concrete(), &mut func_decl.body);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&type_entry, &body_type) {
                    ctx.diagnostic(err.span(func_decl.symbol.span));
                }
            },
            StmtKind::Return(return_stmt) => todo!(),
            StmtKind::Struct(struct_decl) => todo!(),
            StmtKind::Var(var_decl) => {
                let symbol_id = var_decl.symbol.unwrap_id();

                let type_entry = {
                    let ty_cache = ctx.type_cache.borrow();
                    ty_cache.get(symbol_id).cloned().unwrap()
                };

                let init_type_entry = self.infer_expr(ctx, var_decl.ty.as_ref().map(|ty| &ty.kind), &mut var_decl.initializer);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&type_entry, &init_type_entry) {
                    ctx.diagnostic(err.span(var_decl.symbol.span));
                }
            },
        }
    }

    fn infer_expr(&self, ctx: &mut AnalyzerContext, contextual_type: Option<&TypeKind>, expr: &mut Expr) -> TypeCacheEntry {
        match &mut expr.item {
            ExprKind::Assign(assign_expr) => todo!(),
            ExprKind::Binary(binary_expr) => {
                let left_type = self.infer_expr(ctx, None, &mut binary_expr.left);
                let right_type = self.infer_expr(ctx, None, &mut binary_expr.right);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&left_type, &right_type) {
                    ctx.diagnostic(err.span(binary_expr.operator.span));
                }

                left_type
            },
            ExprKind::Block(block_expr) => {
                let mut last_type = TypeCacheEntry::Concrete(TypeKind::Unit);

                for stmt in &mut block_expr.statements {
                    self.infer_stmt(ctx, contextual_type, stmt);
                }

                if let Some(expr) = &mut block_expr.tail_expr {
                    last_type = self.infer_expr(ctx, contextual_type, expr);
                }
                
                last_type
            },
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
                let ty = TypeInference::infer_literal_type(contextual_type, literal_expr)
                    .unwrap_or(TypeKind::Unit);

                TypeCacheEntry::Concrete(ty)
            },
            ExprKind::Struct(struct_expr) => todo!(),
            ExprKind::TupleLiteral(tuple_expr) => todo!(),
            ExprKind::Unary(unary_expr) => todo!(),
        }
    }
}