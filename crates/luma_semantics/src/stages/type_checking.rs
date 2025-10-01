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
        HirStatementKind::Scope { statements } => {
            for stmt in statements {
                analyze_stmt(ctx, stmt);
            }

            TypeKind::Void
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