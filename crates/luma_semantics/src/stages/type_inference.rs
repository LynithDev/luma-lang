use luma_diagnostic::DiagnosticResult;

use crate::{hir::prelude::*, AnalyzerContext, AnalyzerStage};

pub struct TypeInferenceStage;

impl AnalyzerStage for TypeInferenceStage {
    fn name(&self) -> &str {
        "TypeInference"
    }

    fn run(&mut self, ctx: &mut AnalyzerContext) -> bool {
        for statement in ctx.input.code.borrow_mut().as_hir_mut_unchecked().statements.iter() {
            analyze_stmt(ctx, statement);
        }

        true
    }
}

fn analyze_stmt(ctx: &mut AnalyzerContext, stmt: &HirStatement) {
    if let Err(err) = try_analyze_stmt(ctx, stmt) {
        ctx.reporter.report(err);
    }
}

fn try_analyze_stmt(ctx: &mut AnalyzerContext, stmt: &HirStatement) -> DiagnosticResult<()> {
    match &stmt.kind {
        HirStatementKind::VarDecl(decl) => {
            // early check to prevent having to look up the symbol again
            if decl
                .ty
                .kind != TypeKind::Unknown
            {
                type_check_scope(ctx, &decl.value);
                return Ok(());
            }
            
            let inferred_type = if let Some(value) = &decl.value {
                infer_expr_type(ctx, value)
            } else {
                TypeKind::Unknown
            };
            
            let Some(symbol) = ctx.symbol_table.value_table.lookup_id_mut(decl.symbol_id) else {
                unreachable!("variable symbol id should always be valid here");
            };

            symbol.ty = inferred_type;
        }
        HirStatementKind::FuncDecl(decl) => {
            // early check to prevent lookups
            if decl
                .return_type
                .kind != TypeKind::Unknown
            {
                type_check_scope(ctx, &decl.body);
                return Ok(());
            }

            // we enter scope for the function parameters as they are scoped to the func body
            ctx.symbol_table.enter_scope();

            let inferred_type = if let Some(body) = &decl.body {
                if let HirExpressionKind::Scope { statements } = &body.kind {
                    // we don't want to enter a new scope here
                    infer_scope_type(ctx, statements)
                } else {
                    infer_expr_type(ctx, body)
                }
            } else {
                TypeKind::Void
            };

            ctx.symbol_table.leave_scope();

            let Some(symbol) = ctx.symbol_table.value_table.lookup_id_mut(decl.symbol_id) else {
                unreachable!("function symbol id should always be valid here");
            };

            symbol.ty = inferred_type;
        }
        HirStatementKind::ClassDecl(_) => {
            todo!("Class declaration")
        }
        _ => {}
    }

    Ok(())
}

fn type_check_scope(ctx: &mut AnalyzerContext, expr: &Option<Box<HirExpression>>) {
    if let Some(value) = &expr
        && let HirExpressionKind::Scope { statements } = &value.kind
    {
        ctx.symbol_table.enter_scope();
        infer_scope_type(ctx, statements);
        ctx.symbol_table.leave_scope();
    }
}

fn infer_expr_type(ctx: &mut AnalyzerContext, expression: &HirExpression) -> TypeKind {
    match &expression.kind {
        HirExpressionKind::Literal { kind } => match kind {
            HirLiteralKind::String(_) => TypeKind::String,
            HirLiteralKind::Boolean(_) => TypeKind::Boolean,
            HirLiteralKind::Integer(HirLiteralIntegerKind::Int8(_)) => TypeKind::Int8,
            HirLiteralKind::Integer(HirLiteralIntegerKind::Int16(_)) => TypeKind::Int16,
            HirLiteralKind::Integer(HirLiteralIntegerKind::Int32(_)) => TypeKind::Int32,
            HirLiteralKind::Integer(HirLiteralIntegerKind::Int64(_)) => TypeKind::Int64,
            HirLiteralKind::Integer(HirLiteralIntegerKind::UInt8(_)) => TypeKind::UInt8,
            HirLiteralKind::Integer(HirLiteralIntegerKind::UInt16(_)) => TypeKind::UInt16,
            HirLiteralKind::Integer(HirLiteralIntegerKind::UInt32(_)) => TypeKind::UInt32,
            HirLiteralKind::Integer(HirLiteralIntegerKind::UInt64(_)) => TypeKind::UInt64,
            HirLiteralKind::Float(HirLiteralFloatKind::Float32(_)) => TypeKind::Float32,
            HirLiteralKind::Float(HirLiteralFloatKind::Float64(_)) => TypeKind::Float64,
        },

        HirExpressionKind::Unary { operator, value } => {
            let expr_type = infer_expr_type(ctx, value);

            match operator {
                UnaryOperator::Negative => expr_type.as_signed(),
                UnaryOperator::Not => TypeKind::Boolean,
                _ => expr_type,
            }
        }

        HirExpressionKind::Binary {
            left,
            operator: _,
            right,
        } => {
            let left_type = infer_expr_type(ctx, left);
            let right_type = infer_expr_type(ctx, right);

            let lhs_prec = left_type.precedence();
            let rhs_prec = right_type.precedence();

            // if the left is of lower precedence (higher priority), choose that one
            // else always choose right
            if lhs_prec < rhs_prec {
                left_type
            } else {
                right_type
            }
        }

        HirExpressionKind::Comparison { .. } => TypeKind::Boolean,

        HirExpressionKind::ArrayGet { array, .. } => {
            let array_type = infer_expr_type(ctx, array);

            if array_type.is_array()
                && let TypeKind::Array(elem_ty) = array_type
            {
                *elem_ty
            } else {
                TypeKind::Unknown
            }
        }

        HirExpressionKind::Get { object, .. } => {
            let object_type = infer_expr_type(ctx, object);

            if let TypeKind::Object(_) = object_type {
                // if let Some(field) = fields.get(property) {
                //     return field.clone();
                // }
                panic!("Object field lookup not implemented yet");
            }

            TypeKind::Unknown
        }

        HirExpressionKind::Group { inner } => infer_expr_type(ctx, inner),

        HirExpressionKind::If { .. } => {
            todo!("If expression type inference not implemented yet");
        }

        HirExpressionKind::Invoke {
            callee,
            arguments: _,
        } => {
            infer_expr_type(ctx, callee)
        }

        HirExpressionKind::Logical { .. } => TypeKind::Boolean,

        HirExpressionKind::Variable { symbol_id } => {
            if let Some(symbol) = ctx.symbol_table.value_table.lookup_id(*symbol_id) {
                symbol.ty.clone()
            } else {
                TypeKind::Unknown
            }
        }

        HirExpressionKind::Scope { statements } => {
            ctx.symbol_table.enter_scope();
            let ty = infer_scope_type(ctx, statements);
            ctx.symbol_table.leave_scope();

            ty
        }

        _ => {
            dbg!("handle {} kind", &expression.kind);
            TypeKind::Unknown
        }
    }
}

fn infer_scope_type(ctx: &mut AnalyzerContext, statements: &[HirStatement]) -> TypeKind {
    let mut found_type = TypeKind::Void;

    for stmt in statements {
        analyze_stmt(ctx, stmt);

        let expr = match &stmt.kind {
            HirStatementKind::Return { value: Some(value) } => value,
            HirStatementKind::Expression { inner } => inner,
            _ => continue,
        };

        found_type = infer_expr_type(ctx, expr);
        break;
    }

    found_type
}
