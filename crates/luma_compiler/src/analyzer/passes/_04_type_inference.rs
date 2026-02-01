use luma_core::{Operator, ast::*};
use luma_diagnostic::LumaError;

use crate::analyzer::{AnalyzerContext, AnalyzerStage, error::AnalyzerErrorKind};

pub struct TypeInference;

impl AnalyzerStage for TypeInference {
    fn name(&self) -> String {
        String::from("type_inference")
    }

    fn analyze(&mut self, ctx: &AnalyzerContext, input: &mut Ast) {
        self.traverse(
            ctx,
            input,
        );
    }
}

struct TypeContext {
    contextual_type: Option<TypeKind>,
}

impl AstVisitor<'_> for TypeInference {
    type Ctx = AnalyzerContext;

    fn visit_stmt(&mut self, ctx: &Self::Ctx, stmt: &mut Stmt) {
        match &mut stmt.item {
            StmtKind::Var(var_decl) => {
                let type_ctx = TypeContext {
                    contextual_type: var_decl.ty.clone().map(Into::<TypeKind>::into),
                };

                let inferred_ty = Self::infer_expr(
                    ctx,
                    &type_ctx,
                    &mut var_decl.initializer,
                );

                var_decl.initializer.set_type(inferred_ty.clone());
                var_decl.ty = Some(Type::unspanned(inferred_ty));
            },
            StmtKind::Func(_) => {
                
            },
            _ => {}
        }
    }
}

impl TypeInference {
    
    fn infer_expr(
        ctx: &AnalyzerContext, 
        type_ctx: &TypeContext,
        expr: &mut Expr, 
    ) -> TypeKind {

        let ty = match &mut expr.item {
            ExprKind::Literal(literal_expr) => {
                match Self::infer_literal(type_ctx, literal_expr) {
                    Some(ty) => ty,
                    None => {
                        // type inference failure
                        // report error and return unit type as fallback
                        ctx.error(LumaError::new(
                            AnalyzerErrorKind::TypeInferenceFailure, 
                            expr.span,
                        ));

                        TypeKind::Unit
                    }
                }
            },
            ExprKind::Group(group_expr) => {
                Self::infer_expr(ctx, type_ctx, group_expr)
            },
            ExprKind::Binary(binary_expr) => {
                let lty = Self::infer_expr(ctx, type_ctx, &mut binary_expr.left);
                let rty = Self::infer_expr(ctx, type_ctx, &mut binary_expr.right);

                match TypeKind::promote(&lty, &rty) {
                    Some(ty) => ty,
                    None => {
                        // type mismatch
                        ctx.error(LumaError::new(
                            AnalyzerErrorKind::TypeMismatch { 
                                expected: lty,
                                found: rty
                            },
                            binary_expr.left.span.merged(&binary_expr.right.span),
                        ));

                        TypeKind::Unit
                    }
                }
            },
            ExprKind::Unary(unary_expr) => {
                let val_ty = Self::infer_expr(ctx, type_ctx, &mut unary_expr.value);

                match unary_expr.operator.item {
                    Operator::Subtract | Operator::Not => val_ty.clone(),
                    _ => val_ty.clone(),
                }
            },
            // ExprKind::Struct(struct_expr) => {
                // let ty = TypeKind::Named(struct_expr.symbol.name().to_string());

                // // infer types for each field
                // for field in &mut struct_expr.fields {
                //     let field_ty = Self::infer_expr(ctx, type_ctx, &mut field.value);
                //     field.value.set_type(field_ty);
                // }

                // ty
            //     todo!("handle struct expression type inference")
            // }
            
            _ => {
                tracing::warn!("handle expression type inference for {:?}", expr.item);
                expr.ty.clone().unwrap_or(TypeKind::Unit)
            },
        };

        expr.set_type(ty.clone());
        ty
    }

    fn infer_literal(type_ctx: &TypeContext, lit: &LiteralExpr) -> Option<TypeKind> {
        Some(if let Some(contextual) = &type_ctx.contextual_type {

            // contextual type
            match (lit, contextual) {
                // integer literals
                (LiteralExpr::Int(n), TypeKind::UInt8) if *n <= u8::MAX as u64 => TypeKind::UInt8,
                (LiteralExpr::Int(n), TypeKind::UInt16) if *n <= u16::MAX as u64 => TypeKind::UInt16,
                (LiteralExpr::Int(n), TypeKind::UInt32) if *n <= u32::MAX as u64 => TypeKind::UInt32,
                (LiteralExpr::Int(_), TypeKind::UInt64) => TypeKind::UInt64,

                (LiteralExpr::Int(n), TypeKind::Int8) if *n <= i8::MAX as u64 => TypeKind::Int8,
                (LiteralExpr::Int(n), TypeKind::Int16) if *n <= i16::MAX as u64 => TypeKind::Int16,
                (LiteralExpr::Int(n), TypeKind::Int32) if *n <= i32::MAX as u64 => TypeKind::Int32,
                (LiteralExpr::Int(_), TypeKind::Int64) => TypeKind::Int64,

                // float literals
                (LiteralExpr::Float(n), TypeKind::Float32) if *n <= f32::MAX as f64 => TypeKind::Float32,
                (LiteralExpr::Float(_), TypeKind::Float64) => TypeKind::Float64,
                
                // boolean literals
                (LiteralExpr::Bool(_), TypeKind::Bool) => TypeKind::Bool,

                // string literals
                (LiteralExpr::String(_), TypeKind::String) => TypeKind::String,

                // char literals
                (LiteralExpr::Char(_), TypeKind::Char) => TypeKind::Char,
                (LiteralExpr::Char(_), contextual) if contextual.is_uint() => contextual.clone(),

                (_, TypeKind::Unit) => TypeKind::Unit,

                _ => {
                    // type mismatch
                    return None;
                }
            }

        } else {

            // inferred type
            match lit {
                // integer literals
                LiteralExpr::Int(n) if *n <= i32::MAX as u64 => TypeKind::Int32,
                LiteralExpr::Int(n) if *n <= i64::MAX as u64 => TypeKind::Int64,
                LiteralExpr::Int(_) => TypeKind::UInt64,

                // float literals
                LiteralExpr::Float(n) if *n <= f32::MAX as f64 => TypeKind::Float32,
                LiteralExpr::Float(_) => TypeKind::Float64,

                // boolean literals
                LiteralExpr::Bool(_) => TypeKind::Bool,

                // string literals
                LiteralExpr::String(_) => TypeKind::String,

                // char literals
                LiteralExpr::Char(_) => TypeKind::Char,

                LiteralExpr::Unit => TypeKind::Unit,
            }

        })
    }
    
}