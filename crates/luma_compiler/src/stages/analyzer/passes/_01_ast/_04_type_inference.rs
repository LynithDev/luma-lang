use luma_diagnostic::context;

use crate::stages::analyzer::type_cache::TypeCacheEntry;
use crate::{TypeKind, ast::*};

use crate::stages::analyzer::{AnalyzerContext, AnalyzerErrorContext, AnalyzerPass};

pub struct TypeInference;

impl AnalyzerPass<Ast> for TypeInference {
    fn name(&self) -> String {
        String::from("type_inference")
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
impl TypeInference {
    fn infer_stmt(
        &self,
        ctx: &mut AnalyzerContext,
        contextual: &TypeCacheEntry,
        stmt: &mut Stmt,
    ) {
        match &mut stmt.item {
            StmtKind::Expr(expr) => {
                self.infer_expr(ctx, contextual, expr);
            }
            StmtKind::Func(func_decl) => {
                let symbol_id = func_decl.symbol.unwrap_id();

                let type_entry = {
                    let mut ty_cache = ctx.type_cache.borrow_mut();
                    if let Some(ret_type) = &func_decl.return_type {
                        ty_cache.insert_concrete(symbol_id, ret_type.kind.clone());
                        TypeCacheEntry::Concrete(ret_type.kind.clone())
                    } else {
                        let id = ty_cache.insert_relative(symbol_id);
                        TypeCacheEntry::Relative(id)
                    }
                };

                let body_type = self.infer_expr(ctx, &type_entry, &mut func_decl.body);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&type_entry, &body_type) {
                    ctx.diagnostic(
                        err.maybe_span(func_decl.return_type.as_ref().and_then(|r| r.span))
                            .context(context!(
                                AnalyzerErrorContext::BlockContext,
                                match &func_decl.body.item {
                                    ExprKind::Block(block_expr) => block_expr
                                        .tail_expr
                                        .as_ref()
                                        .map(|e| e.span)
                                        .unwrap_or_else(|| block_expr
                                            .statements
                                            .last()
                                            .map(|s| s.span)
                                            .unwrap_or(func_decl.symbol.span)),
                                    _ => func_decl.body.span,
                                }
                            )),
                    );
                }
            }
            StmtKind::Return(_) => todo!(),
            StmtKind::Struct(_) => todo!(),
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

                let init_type = self.infer_expr(ctx, &type_entry, &mut var_decl.initializer);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&type_entry, &init_type) {
                    ctx.diagnostic(err.span(var_decl.symbol.span));
                }

                if let Some(concrete_ty) = init_type.as_concrete() {
                    ctx.type_cache
                        .borrow_mut()
                        .insert_concrete(symbol_id, concrete_ty.clone());
                }
            }
        }
    }

    fn infer_expr(
        &self,
        ctx: &mut AnalyzerContext,
        contextual: &TypeCacheEntry,
        expr: &mut Expr,
    ) -> TypeCacheEntry {
        match &mut expr.item {
            ExprKind::Assign(_) => todo!(),
            ExprKind::Binary(binary_expr) => {
                let left_type = self.infer_expr(ctx, contextual, &mut binary_expr.left);
                let right_type = self.infer_expr(ctx, contextual, &mut binary_expr.right);

                if let Err(err) = ctx.type_cache.borrow_mut().unify(&left_type, &right_type) {
                    ctx.diagnostic(err.span(binary_expr.operator.span));
                }

                left_type
            }
            ExprKind::Block(block_expr) => {
                for stmt in &mut block_expr.statements {
                    self.infer_stmt(ctx, contextual, stmt);
                }

                if let Some(expr) = &mut block_expr.tail_expr {
                    self.infer_expr(ctx, contextual, expr)
                } else {
                    TypeCacheEntry::Concrete(TypeKind::Unit)
                }
            }
            ExprKind::Call(_) => todo!(),
            ExprKind::Get(_) => todo!(),
            ExprKind::Group(expr) => self.infer_expr(ctx, contextual, expr),
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
            ExprKind::If(_) => todo!(),
            ExprKind::Literal(lit) => {
                Self::infer_literal_type(ctx, contextual, lit)
            }
            ExprKind::Struct(_) => todo!(),
            ExprKind::TupleLiteral(_) => todo!(),
            ExprKind::Unary(_) => todo!(),
        }
    }

    pub(super) fn infer_literal_type(
        ctx: &mut AnalyzerContext,
        contextual_type: &TypeCacheEntry,
        lit: &LiteralExpr,
    ) -> TypeCacheEntry {
        if let TypeCacheEntry::Relative(id) = contextual_type {
            return contextual_type.clone();
        }

        TypeCacheEntry::Concrete(match (lit, contextual_type.as_concrete()) {
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
                println!("handle type mismatch errors");
                return contextual_type.clone();
            }
        })
    }
}
