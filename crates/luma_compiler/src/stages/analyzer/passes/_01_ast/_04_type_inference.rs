use crate::stages::analyzer::type_cache::TypeCacheEntry;
use crate::{TypeKind, ast::*};

use crate::stages::analyzer::{AnalyzerContext, AnalyzerPass};

pub struct TypeInference;

impl AnalyzerPass<Ast> for TypeInference {
    fn name(&self) -> String {
        String::from("type_inference")
    }

    fn analyze(&self, ctx: &mut AnalyzerContext, input: &mut Ast) {
        self.traverse(ctx, input);
    }
}

impl AstVisitor<'_> for TypeInference {
    type Ctx = AnalyzerContext;

    fn leave_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {
        self.infer_stmt(ctx, None, stmt);
    }
}

#[allow(unused)]
impl TypeInference {
    fn infer_stmt(
        &self,
        ctx: &mut AnalyzerContext,
        contextual_type: Option<&TypeKind>,
        stmt: &mut Stmt,
    ) {
        match &mut stmt.item {
            StmtKind::Expr(expr) => {
                self.infer_expr(ctx, contextual_type, expr);
            }
            StmtKind::Func(func_decl) => {
                let symbol_id = func_decl.symbol.unwrap_id();

                let type_entry = {
                    let mut ty_cache = ctx.type_cache.borrow_mut();

                    if let Some(ty) = &func_decl.return_type {
                        ty_cache.insert_concrete(symbol_id, ty.kind.clone());
                        TypeCacheEntry::Concrete(ty.kind.clone())
                    } else {
                        let id = ty_cache.insert_relative(symbol_id);
                        TypeCacheEntry::Relative(id)
                    }
                };

                let body_type = self.infer_expr(ctx, type_entry.as_concrete(), &mut func_decl.body);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&type_entry, &body_type) {
                    ctx.diagnostic(err.span(func_decl.symbol.span));
                }
            }
            StmtKind::Return(return_stmt) => todo!(),
            StmtKind::Struct(struct_decl_stmt) => todo!(),
            StmtKind::Var(var_decl) => {
                let symbol_id = var_decl.symbol.unwrap_id();

                let type_entry = {
                    let mut ty_cache = ctx.type_cache.borrow_mut();

                    if let Some(ty) = &var_decl.ty {
                        ty_cache.insert_concrete(symbol_id, ty.kind.clone());
                        TypeCacheEntry::Concrete(ty.kind.clone())
                    } else {
                        let id = ty_cache.insert_relative(symbol_id);
                        TypeCacheEntry::Relative(id)
                    }
                };

                let init_type =
                    self.infer_expr(ctx, type_entry.as_concrete(), &mut var_decl.initializer);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&type_entry, &init_type) {
                    ctx.diagnostic(err.span(var_decl.symbol.span));
                }
            }
        }
    }

    fn infer_expr(
        &self,
        ctx: &mut AnalyzerContext,
        contextual_type: Option<&TypeKind>,
        expr: &mut Expr,
    ) -> TypeCacheEntry {
        #[allow(unused)]
        match &mut expr.item {
            ExprKind::Assign(assign_expr) => todo!(),
            ExprKind::Binary(binary_expr) => {
                let left_type = self.infer_expr(ctx, None, &mut binary_expr.left);
                let right_type = self.infer_expr(ctx, None, &mut binary_expr.right);

                if let Err(mut err) = ctx.type_cache.borrow_mut().unify(&left_type, &right_type) {
                    ctx.diagnostic(err.span(binary_expr.operator.span));
                }

                left_type
            }
            ExprKind::Block(block_expr) => {
                let mut last_type = TypeCacheEntry::Concrete(TypeKind::Unit);

                for stmt in &mut block_expr.statements {
                    self.infer_stmt(ctx, contextual_type, stmt);
                }

                if let Some(expr) = &mut block_expr.tail_expr {
                    last_type = self.infer_expr(ctx, contextual_type, expr);
                }
                
                last_type
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
            ExprKind::If(if_expr) => todo!(),
            ExprKind::Literal(literal_expr) => {
                let ty = Self::infer_literal_type(contextual_type, literal_expr)
                    .unwrap_or(TypeKind::Unit);
                TypeCacheEntry::Concrete(ty)
            }
            ExprKind::Struct(struct_expr) => todo!(),
            ExprKind::TupleLiteral(tuple_expr) => todo!(),
            ExprKind::Unary(unary_expr) => todo!(),
        }
    }

    pub(super) fn infer_literal_type(
        contextual_type: Option<&TypeKind>,
        lit: &LiteralExpr,
    ) -> Option<TypeKind> {
        Some(match (lit, contextual_type) {
            // integer literals
            (LiteralExpr::Int(n), Some(TypeKind::UInt8)) if *n <= u8::MAX as u64 => TypeKind::UInt8,
            (LiteralExpr::Int(n), Some(TypeKind::UInt16)) if *n <= u16::MAX as u64 => {
                TypeKind::UInt16
            }
            (LiteralExpr::Int(n), Some(TypeKind::UInt32)) if *n <= u32::MAX as u64 => {
                TypeKind::UInt32
            }
            (LiteralExpr::Int(_), Some(TypeKind::UInt64)) => TypeKind::UInt64,

            (LiteralExpr::Int(n), Some(TypeKind::Int8)) if *n <= i8::MAX as u64 => TypeKind::Int8,
            (LiteralExpr::Int(n), Some(TypeKind::Int16)) if *n <= i16::MAX as u64 => {
                TypeKind::Int16
            }
            (LiteralExpr::Int(n), Some(TypeKind::Int32)) if *n <= i32::MAX as u64 => {
                TypeKind::Int32
            }
            (LiteralExpr::Int(_), Some(TypeKind::Int64)) => TypeKind::Int64,
            (LiteralExpr::Int(n), None) if *n <= i32::MAX as u64 => TypeKind::Int32,
            (LiteralExpr::Int(n), None) if *n <= i64::MAX as u64 => TypeKind::Int64,
            (LiteralExpr::Int(_), None) => TypeKind::UInt64,

            // float literals
            (LiteralExpr::Float(n), Some(TypeKind::Float32)) if *n <= f32::MAX as f64 => {
                TypeKind::Float32
            }
            (LiteralExpr::Float(_), Some(TypeKind::Float64)) => TypeKind::Float64,
            (LiteralExpr::Float(n), None) if *n <= f32::MAX as f64 => TypeKind::Float32,
            (LiteralExpr::Float(_), None) => TypeKind::Float64,

            // boolean literals
            (LiteralExpr::Bool(_), Some(TypeKind::Bool)) => TypeKind::Bool,
            (LiteralExpr::Bool(_), None) => TypeKind::Bool,

            // string literals
            (LiteralExpr::String(_), Some(TypeKind::String)) => TypeKind::String,
            (LiteralExpr::String(_), None) => TypeKind::String,

            // char literals
            (LiteralExpr::Char(_), Some(TypeKind::Char)) => TypeKind::Char,
            (LiteralExpr::Char(_), Some(contextual)) if contextual.is_uint() => contextual.clone(),
            (LiteralExpr::Char(_), None) => TypeKind::Char,

            // unit literals
            (LiteralExpr::Unit, Some(TypeKind::Unit)) => TypeKind::Unit,
            (LiteralExpr::Unit, _) => TypeKind::Unit,

            _ => {
                // type mismatch
                return None;
            }
        })
    }
}
