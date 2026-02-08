use crate::{OperatorKind, Type, TypeKind, ast::*};
use luma_diagnostic::error;

use crate::stages::analyzer::{AnalyzerContext, AnalyzerError, AnalyzerPass};

pub struct TypeInference;

impl AnalyzerPass<Ast> for TypeInference {
    fn name(&self) -> String {
        String::from("type_inference")
    }

    fn analyze(&self, ctx: &mut AnalyzerContext, input: &mut Ast) {
        self.traverse(ctx, input);
    }
}

#[derive(Default)]
struct TypeContext {
    contextual_type: Option<TypeKind>,
}

impl AstVisitor<'_> for TypeInference {
    type Ctx = AnalyzerContext;

    fn visit_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {
        match &mut stmt.item {
            StmtKind::Expr(expr) => {
                Self::infer_expr(ctx, &TypeContext::default(), expr);
            }
            StmtKind::Func(_) => todo!(),
            StmtKind::Return(return_stmt) => todo!(),
            StmtKind::Struct(struct_decl_stmt) => todo!(),
            StmtKind::Var(var_decl) => {
                let type_ctx = TypeContext {
                    contextual_type: var_decl.ty.clone().map(Into::<TypeKind>::into),
                };

                let inferred_ty = Self::infer_expr(ctx, &type_ctx, &mut var_decl.initializer);

                var_decl.initializer.set_type(inferred_ty.clone());
                var_decl.ty = Some(Type::unspanned(inferred_ty));
            }
        }
    }
}

impl TypeInference {
    fn infer_expr(ctx: &AnalyzerContext, type_ctx: &TypeContext, expr: &mut Expr) -> TypeKind {
        if let Some(ty) = &expr.ty {
            tracing::debug!(
                "infer_expr: type already known for {:?} is {:?}",
                &expr.item,
                ty
            );
            return ty.clone();
        }

        let ty: Option<TypeKind> = match &mut expr.item {
            ExprKind::Literal(literal_expr) => Self::infer_literal(type_ctx, literal_expr),
            ExprKind::Group(group_expr) => Some(Self::infer_expr(ctx, type_ctx, group_expr)),
            ExprKind::Binary(binary_expr) => {
                let lty = Self::infer_expr(ctx, type_ctx, &mut binary_expr.left);
                let rty = Self::infer_expr(ctx, type_ctx, &mut binary_expr.right);

                println!("lty: {:?}, rty: {:?}", lty, rty);

                TypeKind::promote(&lty, &rty)
            }
            ExprKind::Unary(unary_expr) => {
                Some(Self::infer_expr(ctx, type_ctx, &mut unary_expr.value))
            }
            ExprKind::Ident(ident_expr) => {
                if let Some(symbol_id) = ident_expr.symbol.id() {
                    if let Some(symbol) = ctx.symbols.borrow().get_symbol(symbol_id) {
                        symbol.declared_ty.clone().map(|t| t.kind)
                    } else {
                        ctx.diagnostic(error!(
                            AnalyzerError::UnidentifiedSymbol {
                                name: ident_expr.symbol.name().to_string()
                            },
                            expr.span
                        ));

                        None
                    }
                } else {
                    ctx.diagnostic(error!(
                        AnalyzerError::UnidentifiedSymbol {
                            name: ident_expr.symbol.name().to_string()
                        },
                        expr.span
                    ));

                    None
                }
            }
            _ => {
                tracing::warn!("handle expression type inference for {:?}", expr.item);
                expr.ty.clone()
            }
        };

        let ty = ty.unwrap_or(TypeKind::Unit);

        expr.set_type(ty.clone());
        ty
    }

    fn infer_literal(type_ctx: &TypeContext, lit: &LiteralExpr) -> Option<TypeKind> {
        Some(if let Some(contextual) = &type_ctx.contextual_type {
            // contextual type
            match (lit, contextual) {
                // integer literals
                (LiteralExpr::Int(n), TypeKind::UInt8) if *n <= u8::MAX as u64 => TypeKind::UInt8,
                (LiteralExpr::Int(n), TypeKind::UInt16) if *n <= u16::MAX as u64 => {
                    TypeKind::UInt16
                }
                (LiteralExpr::Int(n), TypeKind::UInt32) if *n <= u32::MAX as u64 => {
                    TypeKind::UInt32
                }
                (LiteralExpr::Int(_), TypeKind::UInt64) => TypeKind::UInt64,

                (LiteralExpr::Int(n), TypeKind::Int8) if *n <= i8::MAX as u64 => TypeKind::Int8,
                (LiteralExpr::Int(n), TypeKind::Int16) if *n <= i16::MAX as u64 => TypeKind::Int16,
                (LiteralExpr::Int(n), TypeKind::Int32) if *n <= i32::MAX as u64 => TypeKind::Int32,
                (LiteralExpr::Int(_), TypeKind::Int64) => TypeKind::Int64,

                // float literals
                (LiteralExpr::Float(n), TypeKind::Float32) if *n <= f32::MAX as f64 => {
                    TypeKind::Float32
                }
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
