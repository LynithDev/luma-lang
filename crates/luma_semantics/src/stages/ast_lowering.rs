use crate::{AnalyzerDiagnostic, hir::prelude::*};
use luma_core::{
    Cursor, Span, SymbolId,
    ast::{AstSymbol, prelude::*},
};

use luma_diagnostic::{DiagnosticReport, DiagnosticResult};

use crate::{AnalyzerContext, AnalyzerStage, ParsedCodeKind};

pub struct AstLoweringStage;

impl AnalyzerStage for AstLoweringStage {
    fn name(&self) -> &str {
        "AstLowering"
    }

    fn run(&mut self, ctx: &mut AnalyzerContext) -> bool {
        let code = ctx.input.code.borrow();
        let statements = &code.as_ast_unchecked().statements;

        let mut hir = Hir::with_capacity(statements.len());

        for statement in statements {
            match ast_to_hir_stmt(ctx, statement) {
                Ok(Some(stmt)) => hir.statements.push(stmt),
                Ok(None) => {}
                Err(err) => {
                    ctx.reporter.report(err);
                }
            }
        }

        // we are going to mutate it and the variable code is still in scope so we drop it
        drop(code);

        ctx.input.code.replace(ParsedCodeKind::Hir(hir));

        true
    }
}

fn ast_to_hir_stmt(ctx: &mut AnalyzerContext, stmt: &Statement) -> DiagnosticResult<Option<HirStatement>> {
    let kind = match &stmt.kind {
        StatementKind::While { .. } => todo!("while statement lowering not implemented"),
        StatementKind::Expression { inner } => HirStatementKind::Expression {
            inner: ast_to_hir_expr(ctx, inner)?,
        },
        StatementKind::Continue { label } => {
            let symbol_id = if let Some(label) = label {
                Some(unwrap_ast_symbol(ctx, label)?)
            } else {
                None
            };

            HirStatementKind::Continue { label: symbol_id }
        }
        StatementKind::Break { label } => {
            let symbol_id = if let Some(label) = label {
                Some(unwrap_ast_symbol(ctx, label)?)
            } else {
                None
            };

            HirStatementKind::Break { label: symbol_id }
        }
        StatementKind::Return { value } => {
            let hir_value = if let Some(value) = value {
                Some(Box::new(ast_to_hir_expr(ctx, value)?))
            } else {
                None
            };

            HirStatementKind::Return { value: hir_value }
        }
        StatementKind::Import { .. } => {
            todo!("import statement lowering not implemnted")
        }
        StatementKind::FuncDecl(func_decl) => {
            let mut parameters: Vec<HirParameter> = Vec::with_capacity(func_decl.parameters.len());
            for param in &func_decl.parameters {
                match unwrap_ast_symbol(ctx, &param.symbol) {
                    Ok(symbol_id) => {
                        parameters.push(HirParameter {
                            symbol_id,
                            mutable: param.mutable,
                            ty: param.ty.clone(),
                            span: param.span,
                            cursor: param.cursor,
                        });
                    }
                    Err(err) => {
                        ctx.reporter.report(err);
                    }
                }
            }

            let body = if let Some(body) = &func_decl.body {
                Some(Box::new(ast_to_hir_expr(ctx, body)?))
            } else {
                None
            };

            let return_type = func_decl.return_type.clone().unwrap_or(Type {
                kind: TypeKind::Unknown,
                span: stmt.span,
                cursor: stmt.cursor,
            });

            HirStatementKind::FuncDecl(HirFuncDecl {
                visibility: func_decl.visibility,
                symbol_id: unwrap_ast_symbol(ctx, &func_decl.symbol)?,
                parameters,
                return_type,
                body,
            })
        }
        StatementKind::VarDecl(var_decl) => {
            let value = if let Some(value) = &var_decl.value {
                Some(Box::new(ast_to_hir_expr(ctx, value)?))
            } else {
                None
            };

            HirStatementKind::VarDecl(HirVarDecl {
                visibility: var_decl.visibility,
                mutable: var_decl.mutable,
                symbol_id: unwrap_ast_symbol(ctx, &var_decl.symbol)?,
                ty: var_decl.ty.clone().unwrap_or(Type {
                    kind: TypeKind::Unknown,
                    span: stmt.span,
                    cursor: stmt.cursor,
                }),
                value,
            })
        }
        StatementKind::ClassDecl(_) => todo!("class lowering not implementat"),
        StatementKind::EndOfFile => {
            return Ok(None);
        },
    };

    Ok(Some(HirStatement {
        kind,
        span: stmt.span,
        cursor: stmt.cursor,
    }))
}

fn ast_to_hir_expr(
    ctx: &mut AnalyzerContext,
    expr: &Expression,
) -> DiagnosticResult<HirExpression> {
    let (kind, ty) = match &expr.kind {
        ExpressionKind::ArrayGet { array, index } => (
            HirExpressionKind::ArrayGet {
                array: Box::new(ast_to_hir_expr(ctx, array)?),
                index: Box::new(ast_to_hir_expr(ctx, index)?),
            },
            TypeKind::Unknown,
        ),
        ExpressionKind::ArraySet {
            array,
            index,
            value,
        } => (
            HirExpressionKind::ArraySet {
                array: Box::new(ast_to_hir_expr(ctx, array)?),
                index: Box::new(ast_to_hir_expr(ctx, index)?),
                value: Box::new(ast_to_hir_expr(ctx, value)?),
            },
            TypeKind::Void,
        ),
        ExpressionKind::Assign {
            symbol,
            operator,
            value,
        } => (
            HirExpressionKind::Assign {
                symbol_id: unwrap_ast_symbol(ctx, symbol)?,
                operator: *operator,
                value: Box::new(ast_to_hir_expr(ctx, value)?),
            },
            TypeKind::Unknown,
        ),
        ExpressionKind::Binary {
            left,
            operator,
            right,
        } => (
            HirExpressionKind::Binary {
                left: Box::new(ast_to_hir_expr(ctx, left)?),
                operator: *operator,
                right: Box::new(ast_to_hir_expr(ctx, right)?),
            },
            TypeKind::Unknown,
        ),
        ExpressionKind::Comparison {
            left,
            operator,
            right,
        } => (
            HirExpressionKind::Comparison {
                left: Box::new(ast_to_hir_expr(ctx, left)?),
                operator: *operator,
                right: Box::new(ast_to_hir_expr(ctx, right)?),
            },
            TypeKind::Boolean,
        ),
        ExpressionKind::Get {
            object,
            property_symbol,
        } => (
            HirExpressionKind::Get {
                object: Box::new(ast_to_hir_expr(ctx, object)?),
                property_symbol_id: unwrap_ast_symbol(ctx, property_symbol)?,
            },
            TypeKind::Unknown,
        ),
        ExpressionKind::Group { inner } => (
            HirExpressionKind::Group {
                inner: Box::new(ast_to_hir_expr(ctx, inner)?),
            },
            TypeKind::Unknown,
        ),
        ExpressionKind::If {
            main_branch: main_expr,
            branches,
            else_branch: else_expr,
        } => {
            let hir_main = Box::new(HirConditionalBranch {
                condition: ast_to_hir_expr(ctx, &main_expr.condition)?,
                body: ast_to_hir_expr(ctx, &main_expr.body)?,
            });

            let hir_branches = if let Some(branches) = branches {
                let mut hir_branches = Vec::with_capacity(branches.len());
                for branch in branches {
                    hir_branches.push(HirConditionalBranch {
                        condition: ast_to_hir_expr(ctx, &branch.condition)?,
                        body: ast_to_hir_expr(ctx, &branch.body)?,
                    });
                }
                Some(hir_branches)
            } else {
                None
            };

            let hir_else = if let Some(else_expr) = else_expr {
                Some(Box::new(ast_to_hir_expr(ctx, else_expr)?))
            } else {
                None
            };

            (
                HirExpressionKind::If {
                    main_expr: hir_main,
                    branches: hir_branches,
                    else_expr: hir_else,
                },
                TypeKind::Unknown,
            )
        }
        ExpressionKind::Invoke { callee, arguments } => {
            let mut hir_arguments = Vec::with_capacity(arguments.capacity());
            for arg in arguments {
                hir_arguments.push(ast_to_hir_expr(ctx, arg)?);
            }
            (
                HirExpressionKind::Invoke {
                    callee: Box::new(ast_to_hir_expr(ctx, callee)?),
                    arguments: hir_arguments,
                },
                TypeKind::Unknown,
            )
        }
        ExpressionKind::Literal { kind, value } => {
            let kind = ast_to_hir_literal(kind, value)?;
            let ty = TypeKind::from(&kind);

            (HirExpressionKind::Literal { kind }, ty)
        }
        ExpressionKind::Logical {
            left,
            operator,
            right,
        } => (
            HirExpressionKind::Logical {
                left: Box::new(ast_to_hir_expr(ctx, left)?),
                operator: *operator,
                right: Box::new(ast_to_hir_expr(ctx, right)?),
            },
            TypeKind::Boolean,
        ),
        ExpressionKind::Scope { statements, block_value: value } => {
            let mut hir_statements = Vec::with_capacity(statements.len());
            for statement in statements {
                if let Some(hir_stmt) = ast_to_hir_stmt(ctx, statement)? {
                    hir_statements.push(hir_stmt);
                }
            }

            let value = if let Some(value) = value {
                Some(Box::new(ast_to_hir_expr(ctx, value)?))
            } else {
                None
            };

            (
                HirExpressionKind::Scope {
                    statements: hir_statements,
                    value
                },
                TypeKind::Unknown,
            )
        }
        ExpressionKind::Unary { operator, value } => (
            HirExpressionKind::Unary {
                operator: *operator,
                value: Box::new(ast_to_hir_expr(ctx, value)?),
            },
            TypeKind::Unknown,
        ),
        ExpressionKind::Variable { symbol } => {
            // dbg!(&ctx.symbol_table);
            // dbg!(symbol);
            
            (
            HirExpressionKind::Variable {
                symbol_id: unwrap_ast_symbol(ctx, symbol)?,
            },
            TypeKind::Unknown,
        )
        },
    };

    Ok(HirExpression {
        kind,
        ty,
        span: expr.span,
        cursor: expr.cursor,
    })
}

fn ast_to_hir_literal(kind: &LiteralKind, value: &str) -> DiagnosticResult<HirLiteralKind> {
    match kind {
        LiteralKind::Integer => Ok(HirLiteralKind::Integer({
            let parsed = value.parse::<u64>().map_err(|_| DiagnosticReport {
                span: Span::default(),
                cursor: Cursor::default(),
                message: Box::new(AnalyzerDiagnostic::InvalidLiteralValue(
                    TypeKind::UInt64,
                    value.to_string(),
                )),
            })?;

            if parsed <= u32::MAX as u64 {
                HirLiteralIntegerKind::UInt32(parsed as u32)
            } else {
                HirLiteralIntegerKind::UInt64(parsed)
            }
        })),
        LiteralKind::Float => Ok({
            let parsed = value.parse::<f64>().map_err(|_| DiagnosticReport {
                span: Span::default(),
                cursor: Cursor::default(),
                message: Box::new(AnalyzerDiagnostic::InvalidLiteralValue(
                    TypeKind::Float64,
                    value.to_string(),
                )),
            })?;

            HirLiteralKind::Float(HirLiteralFloatKind::Float64(parsed))
        }),
        LiteralKind::String => Ok(HirLiteralKind::String(value.to_string())),
        LiteralKind::Boolean => match value {
            "true" => Ok(HirLiteralKind::Boolean(true)),
            "false" => Ok(HirLiteralKind::Boolean(false)),
            _ => Err(DiagnosticReport {
                span: Span::default(),
                cursor: Cursor::default(),
                message: Box::new(AnalyzerDiagnostic::InvalidLiteralValue(
                    TypeKind::Boolean,
                    value.to_string(),
                )),
            }),
        },
    }
}

fn unwrap_ast_symbol(ctx: &mut AnalyzerContext, symbol: &AstSymbol) -> DiagnosticResult<SymbolId> {
    let Some(id) = symbol.id else {
        // unreachable!("tried to unwrap an AstSymbol that has no id: {symbol:#?}");
        dbg!("tried to unwrap an AstSymbol that has no id: {symbol:#?}");
        return Err(DiagnosticReport {
            span: symbol.span,
            cursor: symbol.cursor,
            message: Box::new(AnalyzerDiagnostic::UnresolvedSymbol(symbol.name.clone())),
        });
    };

    if ctx.symbol_table.value_table.lookup_id(id).is_some() {
        Ok(id)
    } else {
        Err(DiagnosticReport {
            span: symbol.span,
            cursor: symbol.cursor,
            message: Box::new(AnalyzerDiagnostic::UnresolvedSymbol(symbol.name.clone())),
        })
    }
}
