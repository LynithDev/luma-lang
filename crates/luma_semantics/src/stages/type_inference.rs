use std::ops::Deref;

use luma_diagnostic::DiagnosticResult;

use crate::{hir::prelude::*, AnalyzerContext, AnalyzerStage};

pub struct TypeInferenceStage;

impl AnalyzerStage for TypeInferenceStage {
    fn name(&self) -> &str {
        "TypeInference"
    }

    fn run(&mut self, ctx: &mut AnalyzerContext) -> bool {
        for statement in ctx.input.code.borrow_mut().as_hir_mut_unchecked().statements.iter_mut() {
            analyze_stmt(ctx, statement);
        }

        true
    }
}

fn analyze_stmt(ctx: &mut AnalyzerContext, stmt: &mut HirStatement) {
    if let Err(err) = try_analyze_stmt(ctx, stmt) {
        ctx.reporter.report(err);
    }
}

fn try_analyze_stmt(ctx: &mut AnalyzerContext, stmt: &mut HirStatement) -> DiagnosticResult<()> {
    match &mut stmt.kind {
        HirStatementKind::VarDecl(decl) => {
            // early check to prevent having to look up the symbol again
            if decl
                .ty
                .kind != TypeKind::Unknown
            {
                infer_scope(ctx, decl.value.as_mut().map(|v| v.as_mut()));
                return Ok(());
            }
            
            let inferred_type = if let Some(value) = &mut decl.value {
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
                infer_scope(ctx, decl.body.as_mut().map(|b| b.as_mut()));
                return Ok(());
            }

            // we enter scope for the function parameters as they are scoped to the func body
            ctx.symbol_table.enter_scope();

            let param_types = decl
                .parameters
                .iter()
                .map(|param| {
                    let Some(symbol) = ctx.symbol_table.value_table.lookup_id(param.symbol_id) else {
                        unreachable!("parameter symbol id should always be valid here");
                    };

                    symbol.ty.clone()
                })
                .collect::<Vec<_>>();

            let return_type = if let Some(body) = &mut decl.body {
                if let HirExpressionKind::Scope { statements } = &mut body.kind {
                    // we don't want to enter a new scope here
                    let ty = infer_statements(ctx, statements);
                    body.ty = ty.clone();
                    ty
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

            symbol.ty = TypeKind::Function { 
                param_types, 
                return_type: Box::new(return_type.clone()) 
            };

            decl.return_type.kind = return_type;
        }
        HirStatementKind::ClassDecl(_) => {
            todo!("Class declaration")
        }
        _ => {}
    }

    Ok(())
}

fn infer_expr_type(ctx: &mut AnalyzerContext, expression: &mut HirExpression) -> TypeKind {
    let ty = match &mut expression.kind {
        HirExpressionKind::Literal { kind } => {
            TypeKind::from(kind.deref())
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

            TypeKind::from_tuple(left_type, right_type)
        }

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

        HirExpressionKind::If { main_expr, branches, else_expr } => {
            // first infer condition
            infer_expr_type(ctx, &mut main_expr.condition);

            // then infer body
            let ty = infer_expr_type(ctx, &mut main_expr.body);

            // infer branches
            if let Some(branches) = branches {
                for branch in branches {
                    infer_expr_type(ctx, &mut branch.condition);
                    infer_expr_type(ctx, &mut branch.body);
                }
            }

            // infer else branch
            if let Some(else_expr) = else_expr {
                infer_expr_type(ctx, else_expr);
            }

            ty
        }

        HirExpressionKind::Invoke {
            callee,
            arguments: _,
        } => {
            let ty = infer_expr_type(ctx, callee);
            if let TypeKind::Function { return_type, .. } = ty {
                *return_type
            } else {
                TypeKind::Unknown
            }
        }

        HirExpressionKind::Variable { symbol_id } => {
            if let Some(symbol) = ctx.symbol_table.value_table.lookup_id(*symbol_id) {
                symbol.ty.clone()
            } else {
                TypeKind::Unknown
            }
        }

        HirExpressionKind::Scope { .. } => {
            infer_scope(ctx, Some(expression));

            return expression.ty.clone();
        }

        _ => return expression.ty.clone()
    };

    expression.ty = ty.clone();
    ty
}

fn infer_scope(ctx: &mut AnalyzerContext, expr: Option<&mut HirExpression>) {
    if let Some(expr) = expr
        && let HirExpressionKind::Scope { statements } = &mut expr.kind
    {
        ctx.symbol_table.enter_scope();
        expr.ty = infer_statements(ctx, statements);
        ctx.symbol_table.leave_scope();
    }
}

fn infer_statements(ctx: &mut AnalyzerContext,  statements: &mut [HirStatement]) -> TypeKind {
    let mut found_type = TypeKind::Void;

    for stmt in statements {
        analyze_stmt(ctx, stmt);

        if let HirStatementKind::Return { value: Some(expr) } = &mut stmt.kind {
            found_type = infer_expr_type(ctx, expr);
            break;
        }

    }

    found_type
}
