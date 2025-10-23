use luma_diagnostic::DiagnosticReport;

use crate::{AnalyzerContext, AnalyzerDiagnostic, AnalyzerStage, hir::prelude::*, symbol::ValueSymbol};

pub struct TypeCheckingStage;

impl AnalyzerStage for TypeCheckingStage {
    fn name(&self) -> &str {
        "TypeChecking"
    }

    fn run(&mut self, ctx: &mut AnalyzerContext) -> bool {
        for statement in ctx.input.code.borrow().as_hir_unchecked().statements.iter() {
            analyze_stmt(ctx, statement);
        }
        
        true
    }
}

fn analyze_stmt(ctx: &mut AnalyzerContext, statement: &HirStatement) -> TypeKind {
    match &statement.kind {
        HirStatementKind::Expression { inner } => {
            analyze_expr(ctx, inner)
        },
        HirStatementKind::VarDecl(decl) => {
            if let Some(value) = &decl.value {
                analyze_expr(ctx, value)
            } else {
                TypeKind::Unknown
            }
        },
        HirStatementKind::FuncDecl(decl) => {
            if let Some(body) = &decl.body {
                analyze_expr(ctx, body)
            } else {
                TypeKind::Void
            }
        }
        HirStatementKind::Return { value } => {
            if let Some(value) = value {
                analyze_expr(ctx, value)
            } else {
                TypeKind::Void
            }
        },
        _ => {
            // dbg!("Type checking for statement kind {:?}", &statement.kind);
            TypeKind::Void
        },
    }
}

fn analyze_expr(ctx: &mut AnalyzerContext, expr: &HirExpression) -> TypeKind {
    match &expr.kind {
        HirExpressionKind::Invoke { callee, .. } => {
            if let HirExpressionKind::Variable { symbol_id } = &callee.kind {
                let symbol = ctx.symbol_table.value_table.lookup_id(*symbol_id)
                    .expect("Callee symbol not found in symbol table during type checking")
                    .clone();

                check_arguments(ctx, expr, &symbol);

                if let TypeKind::Function { return_type, .. } = &symbol.ty {
                    *return_type.clone()
                } else {
                    symbol.ty
                }
            } else {
                let ty = callee.ty.clone();
                ctx.reporter.report(DiagnosticReport {
                    message: Box::new(AnalyzerDiagnostic::CalleeNotFunction(ty.clone())),
                    span: expr.span,
                    cursor: expr.cursor,
                });
                ty
            }
        },
        HirExpressionKind::Scope { statements } => {
            for stmt in statements {
                analyze_stmt(ctx, stmt);
            }

            expr.ty.clone()
        },
        HirExpressionKind::If { main_expr, branches, else_expr } => {
            // analyze main branch
            let ret_ty = check_conditional_branch(ctx, main_expr);

            // analyze branches
            if let Some(branches) = branches {
                for branch in branches {
                    let ty = check_conditional_branch(ctx, branch);

                    if ty != ret_ty {
                        ctx.reporter.report(DiagnosticReport {
                            message: Box::new(AnalyzerDiagnostic::ExpectedTypeFoundType(ret_ty.clone(), ty.clone())),
                            span: branch.body.span,
                            cursor: branch.body.cursor,
                        });
                    }
                }
            }

            // analyze else branch
            if let Some(else_expr) = else_expr {
                let ty = analyze_expr(ctx, else_expr);

                if ty != ret_ty {
                    ctx.reporter.report(DiagnosticReport {
                        message: Box::new(AnalyzerDiagnostic::ExpectedTypeFoundType(ret_ty.clone(), ty.clone())),
                        span: else_expr.span,
                        cursor: else_expr.cursor,
                    });
                }
            }

            // if it returns void, it means only 1 branch is required. otherwise, there needs to be at least an else branch
            let returns_type = ret_ty != TypeKind::Void;
            let has_else_branch = else_expr.is_some() || branches.as_ref().is_some_and(|b| !b.is_empty());

            if returns_type && !has_else_branch {
                ctx.reporter.report(DiagnosticReport {
                    message: Box::new(AnalyzerDiagnostic::MissingElseBranch(ret_ty.clone())),
                    span: expr.span,
                    cursor: expr.cursor,
                });
            }

            ret_ty
        },
        _ => expr.ty.clone(),
    }
}

fn check_arguments(ctx: &mut AnalyzerContext, expr: &HirExpression, symbol: &ValueSymbol) {
    let HirExpressionKind::Invoke { arguments, .. } = &expr.kind else {
        return;
    };

    let TypeKind::Function { param_types, .. } = &symbol.ty else {
        ctx.reporter.report(DiagnosticReport {
            message: Box::new(AnalyzerDiagnostic::CalleeNotFunction(symbol.ty.clone())),
            span: expr.span,
            cursor: expr.cursor,
        });
        return;
    };

    if param_types.len() != arguments.len() {
        ctx.reporter.report(DiagnosticReport {
            message: Box::new(AnalyzerDiagnostic::InvalidAmountOfArguments(param_types.len(), arguments.len())),
            span: expr.span,
            cursor: expr.cursor,
        });
        
        return;
    }

    for (arg, expected_ty) in arguments.iter().zip(param_types.iter()) {
        let arg_ty = analyze_expr(ctx, arg);

        if &arg_ty != expected_ty {
            ctx.reporter.report(DiagnosticReport {
                message: Box::new(AnalyzerDiagnostic::MismatchedTypes(expected_ty.clone(), arg_ty)),
                span: arg.span,
                cursor: arg.cursor,
            });
        }
    }
}

fn check_conditional_branch(ctx: &mut AnalyzerContext, branch: &HirConditionalBranch) -> TypeKind {
    let cond_ty = analyze_expr(ctx, &branch.condition);
    if cond_ty != TypeKind::Boolean {
        ctx.reporter.report(DiagnosticReport {
            message: Box::new(AnalyzerDiagnostic::ExpectedTypeFoundType(TypeKind::Boolean, cond_ty.clone())),
            span: branch.condition.span,
            cursor: branch.condition.cursor,
        });
    }

    analyze_expr(ctx, &branch.body)
}