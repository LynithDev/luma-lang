use luma_core::ast::prelude::*;
use luma_diagnostic::{DiagnosticKind, DiagnosticReport};

use crate::{AnalyzerContext, AnalyzerDiagnostic, AnalyzerStage};

pub struct NameResolutionStage;

impl AnalyzerStage for NameResolutionStage {
    fn name(&self) -> &str {
        "NameResolution"
    }

    fn run(&mut self, ctx: &mut AnalyzerContext) -> bool {
        let errors = ctx.reporter.diagnostic_count(DiagnosticKind::Error);
        run_pass(ctx);

        let new_errors = ctx.reporter.diagnostic_count(DiagnosticKind::Error);

        if new_errors > errors {
            return false;
        }

        true
    }
}

fn run_pass(ctx: &mut AnalyzerContext) {
    for statement in ctx
        .input
        .code
        .borrow_mut()
        .as_ast_mut_unchecked()
        .statements
        .iter_mut()
    {
        analyze_stmt(ctx, statement);
    }
}

fn analyze_stmt(ctx: &mut AnalyzerContext, statement: &mut Statement) {
    match &mut statement.kind {
        StatementKind::VarDecl(decl) => {
            // first we do the value
            if let Some(value) = decl.value.as_mut() {
                analyze_expr(ctx, value);
            }

            // then we do the rest
            let ty = decl
                .ty
                .as_ref()
                .map_or(TypeKind::Unknown, |t| t.kind.clone());

            let symbol_id = ctx.symbol_table.declare_value(decl.symbol.name.clone(), ty);
            decl.symbol.id = Some(symbol_id);
        }
        StatementKind::FuncDecl(decl) => {
            let return_type = decl
                .return_type
                .as_ref()
                .map_or(TypeKind::Unknown, |t| t.kind.clone());

            let ty = TypeKind::Function {
                param_types: decl.parameters.iter().map(|p| p.ty.kind.clone()).collect(),
                return_type: Box::new(return_type.clone()),
            };

            let symbol_id = ctx.symbol_table.declare_value(decl.symbol.name.clone(), ty);
            decl.symbol.id = Some(symbol_id);

            // we enter scope for the function parameters as they are scoped to the func body
            ctx.symbol_table.enter_scope();

            for param in &mut decl.parameters {
                let symbol_id = ctx
                    .symbol_table
                    .declare_value(param.symbol.name.clone(), param.ty.kind.clone());

                param.symbol.id = Some(symbol_id);
            }

            // analyze body
            if let Some(body) = decl.body.as_mut() {
                analyze_expr(ctx, body);
            }

            ctx.symbol_table.leave_scope();
        }
        StatementKind::ClassDecl(_) => {
            todo!("class decl name resolution")
        }
        StatementKind::Expression { inner } => {
            analyze_expr(ctx, inner);
        }
        StatementKind::Return { value } => {
            if let Some(expr) = value.as_mut() {
                analyze_expr(ctx, expr);
            }
        }
        StatementKind::EndOfFile => {}
        _ => {
            dbg!("unhandled statement in name resolution: {:?}", statement);
        }
    }
}

fn analyze_expr(ctx: &mut AnalyzerContext, expr: &mut Expression) {
    match &mut expr.kind {
        ExpressionKind::Comparison { left, right, .. }
        | ExpressionKind::Binary { left, right, .. }
        | ExpressionKind::Logical { left, right, .. } => {
            analyze_expr(ctx, left);
            analyze_expr(ctx, right);
        }
        ExpressionKind::Unary { value, .. } => {
            analyze_expr(ctx, value);
        }
        ExpressionKind::Invoke { callee, arguments } => {
            analyze_expr(ctx, callee);

            for arg in arguments {
                analyze_expr(ctx, arg);
            }
        }
        ExpressionKind::Variable { symbol } => {
            if let Some(lookup) = ctx.symbol_table.value_table.lookup_name(&symbol.name) {
                symbol.id = Some(lookup.id);
            } else {
                ctx.reporter.report(DiagnosticReport {
                    message: Box::new(AnalyzerDiagnostic::UnresolvedSymbol(symbol.name.clone())),
                    span: symbol.span,
                    cursor: symbol.cursor,
                });
            }
        }
        ExpressionKind::If {
            main_branch,
            branches,
            else_branch,
        } => {
            analyze_expr(ctx, &mut main_branch.condition);
            analyze_expr(ctx, &mut main_branch.body);

            if let Some(branches) = branches {
                for branch in branches {
                    analyze_expr(ctx, &mut branch.condition);
                    analyze_expr(ctx, &mut branch.body);
                }
            }

            if let Some(else_expr) = else_branch {
                analyze_expr(ctx, else_expr);
            }
        }
        ExpressionKind::Scope { statements, block_value: value } => {
            ctx.symbol_table.enter_scope();

            for stmt in statements {
                analyze_stmt(ctx, stmt);
            }

            if let Some(value) = value {
                analyze_expr(ctx, value);
            }

            ctx.symbol_table.leave_scope();
        }
        ExpressionKind::Assign {
            symbol,
            value,
            ..
        } => {
            analyze_expr(ctx, value);
            
            if let Some(lookup) = ctx.symbol_table.value_table.lookup_name(&symbol.name) {
                symbol.id = Some(lookup.id);
            } else {
                ctx.reporter.report(DiagnosticReport {
                    message: Box::new(AnalyzerDiagnostic::UnresolvedSymbol(symbol.name.clone())),
                    span: symbol.span,
                    cursor: symbol.cursor,
                });
            }
        },
        ExpressionKind::Group { inner } => analyze_expr(ctx, inner),
        ExpressionKind::Literal { .. } => {},
        ExpressionKind::Get { .. } => todo!("property access name resolution"),
        ExpressionKind::ArrayGet { .. } => todo!("array get name resolution"),
        ExpressionKind::ArraySet { .. } => todo!("array set name resolution"),
    }
}
