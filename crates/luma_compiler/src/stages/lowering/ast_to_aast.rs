use luma_core::Span;
use luma_diagnostic::{CompilerResult, error};

use crate::{
    CompilerContext, CompilerStage, TypeKind, aast::*, ast::*, stages::lowering::LoweringError,
};

pub struct AstLoweringStage;

impl CompilerStage<'_> for AstLoweringStage {
    type Input = Vec<Ast>;
    type Output = Vec<AnnotatedAst>;

    fn name() -> &'static str {
        "ast_to_aast"
    }

    fn process(self, ctx: &CompilerContext, inputs: Self::Input) -> Self::Output {
        let mut aasts = Vec::<AnnotatedAst>::new();

        for ast in inputs {
            let aast = match annotate_ast(ast) {
                Ok(aast) => aast,
                Err(err) => {
                    ctx.add_diag(err);
                    return Vec::new();
                }
            };

            aasts.push(aast);
        }

        aasts
    }
}

fn annotate_ast(ast: Ast) -> CompilerResult<AnnotatedAst> {
    Ok(AnnotatedAst {
        statements: ast
            .statements
            .into_iter()
            .map(annotate_stmt)
            .try_collect()?,
        span: ast.span,
    })
}

fn annotate_symbol(symbol: Symbol) -> CompilerResult<AnnotSymbol> {
    Ok(AnnotSymbol {
        name: symbol.name().to_string(),
        id: symbol.unwrap_id(),
        span: symbol.span,
    })
}

fn annotate_stmt(stmt: Stmt) -> CompilerResult<AnnotStmt> {
    Ok(AnnotStmt {
        item: match stmt.item {
            StmtKind::Expr(expr) => AnnotStmtKind::Expr(annotate_expr(expr)?),
            StmtKind::Func(func_decl_stmt) => {
                AnnotStmtKind::Func(annotate_func_decl(func_decl_stmt)?)
            }
            StmtKind::Return(return_stmt) => {
                AnnotStmtKind::Return(annotate_return_stmt(return_stmt)?)
            }
            StmtKind::Struct(struct_decl_stmt) => {
                AnnotStmtKind::Struct(annotate_struct_decl(struct_decl_stmt)?)
            }
            StmtKind::Var(var_decl_stmt) => AnnotStmtKind::Var(annotate_var_decl(var_decl_stmt)?),
        },
        scope_id: stmt
            .scope_id
            .ok_or(error!(LoweringError::MissingScopeId, stmt.span,))?,
        span: stmt.span,
    })
}

fn annotate_func_decl(func_decl: FuncDeclStmt) -> CompilerResult<FuncDeclAnnotStmt> {
    Ok(FuncDeclAnnotStmt {
        visibility: func_decl.visibility,
        parameters: func_decl
            .parameters
            .into_iter()
            .map(|param| {
                Ok(AnnotFuncParam {
                    symbol: annotate_symbol(param.symbol)?,
                    ty: param.ty,
                    default_value: match param.default_value {
                        Some(expr) => Some(annotate_expr(expr)?),
                        None => None,
                    },
                    span: param.span,
                    scope_id: param.scope_id.unwrap(),
                })
            })
            .try_collect()?,
        body: annotate_expr(func_decl.body)?,
        return_type: func_decl
            .return_type
            .ok_or(error!(LoweringError::UnknownType, func_decl.symbol.span,))?,
        symbol: annotate_symbol(func_decl.symbol)?,
    })
}

fn annotate_return_stmt(return_stmt: ReturnStmt) -> CompilerResult<ReturnAnnotStmt> {
    Ok(ReturnAnnotStmt {
        value: match return_stmt.value {
            Some(expr) => Some(annotate_expr(expr)?),
            None => None,
        },
    })
}

fn annotate_struct_decl(struct_decl: StructDeclStmt) -> CompilerResult<StructDeclAnnotStmt> {
    Ok(StructDeclAnnotStmt {
        visibility: struct_decl.visibility,
        symbol: annotate_symbol(struct_decl.symbol)?,
        fields: struct_decl
            .fields
            .into_iter()
            .map(|field| {
                Ok(StructFieldAnnotDecl {
                    visibility: field.visibility,
                    symbol: annotate_symbol(field.symbol)?,
                    ty: field.ty,
                    span: field.span,
                })
            })
            .try_collect()?,
    })
}

fn annotate_var_decl(var_decl: VarDeclStmt) -> CompilerResult<VarDeclAnnotStmt> {
    Ok(VarDeclAnnotStmt {
        visibility: var_decl.visibility,
        ty: var_decl
            .ty
            .ok_or(error!(LoweringError::UnknownType, var_decl.symbol.span))?,
        symbol: annotate_symbol(var_decl.symbol)?,
        initializer: annotate_expr(var_decl.initializer)?,
    })
}

fn annotate_expr(expr: Expr) -> CompilerResult<AnnotExpr> {
    Ok(AnnotExpr {
        item: match expr.item {
            ExprKind::Assign(assign_expr) => AnnotExprKind::Assign(annotate_assign(assign_expr)?),
            ExprKind::Binary(binary_expr) => AnnotExprKind::Binary(annotate_binary(binary_expr)?),
            ExprKind::Block(block_expr) => AnnotExprKind::Block(annotate_block(block_expr)?),
            ExprKind::Call(call_expr) => AnnotExprKind::Call(annotate_call(call_expr)?),
            ExprKind::Get(get_expr) => AnnotExprKind::Get(annotate_get(get_expr)?),
            ExprKind::Group(group_expr) => {
                AnnotExprKind::Group(Box::new(annotate_expr(*group_expr)?))
            }
            ExprKind::Ident(ident_expr) => {
                AnnotExprKind::Ident(annotate_ident(ident_expr, &expr.span)?)
            }
            ExprKind::If(if_expr) => AnnotExprKind::If(annotate_if(if_expr)?),
            ExprKind::Literal(_) => AnnotExprKind::Literal(lower_literal(&expr)?),
            ExprKind::Struct(struct_expr) => AnnotExprKind::Struct(annotate_struct(struct_expr)?),
            ExprKind::TupleLiteral(tuple_expr) => {
                AnnotExprKind::TupleLiteral(annotate_tuple(tuple_expr)?)
            }
            ExprKind::Unary(unary_expr) => AnnotExprKind::Unary(annotate_unary(unary_expr)?),
        },
        ty: expr
            .ty
            .ok_or(error!(LoweringError::UnknownType, expr.span))?,
        scope_id: expr
            .scope_id
            .ok_or(error!(LoweringError::MissingScopeId, expr.span))?,
        span: expr.span,
    })
}

fn annotate_assign(assign_expr: AssignExpr) -> CompilerResult<AssignAnnotExpr> {
    Ok(AssignAnnotExpr {
        target: Box::new(annotate_expr(*assign_expr.target)?),
        operator: assign_expr.operator,
        value: Box::new(annotate_expr(*assign_expr.value)?),
    })
}

fn annotate_binary(binary_expr: BinaryExpr) -> CompilerResult<BinaryAnnotExpr> {
    Ok(BinaryAnnotExpr {
        left: Box::new(annotate_expr(*binary_expr.left)?),
        operator: binary_expr.operator,
        right: Box::new(annotate_expr(*binary_expr.right)?),
    })
}

fn annotate_block(block_expr: BlockExpr) -> CompilerResult<BlockAnnotExpr> {
    Ok(BlockAnnotExpr {
        statements: block_expr
            .statements
            .into_iter()
            .map(annotate_stmt)
            .try_collect()?,
        tail_expr: block_expr
            .tail_expr
            .map(|expr| *expr)
            .map(annotate_expr)
            .transpose()?
            .map(Box::new)
    })
}

fn annotate_call(call_expr: CallExpr) -> CompilerResult<CallAnnotExpr> {
    Ok(CallAnnotExpr {
        callee: Box::new(annotate_expr(*call_expr.callee)?),
        arguments: call_expr
            .arguments
            .into_iter()
            .map(annotate_expr)
            .try_collect()?,
    })
}

fn annotate_get(get_expr: GetExpr) -> CompilerResult<GetAnnotExpr> {
    Ok(GetAnnotExpr {
        object: Box::new(annotate_expr(*get_expr.object)?),
        property: annotate_symbol(get_expr.property)?,
    })
}

fn annotate_ident(ident_expr: IdentExpr, span: &Span) -> CompilerResult<IdentAnnotExpr> {
    Ok(IdentAnnotExpr {
        symbol: AnnotSymbol {
            name: ident_expr.symbol.name().to_string(),
            id: ident_expr.symbol.unwrap_id(),
            span: *span,
        },
    })
}

fn annotate_if(if_expr: IfExpr) -> CompilerResult<IfAnnotExpr> {
    Ok(IfAnnotExpr {
        condition: Box::new(annotate_expr(*if_expr.condition)?),
        then_branch: Box::new(annotate_expr(*if_expr.then_branch)?),
        else_branch: match if_expr.else_branch {
            Some(else_expr) => Some(Box::new(annotate_expr(*else_expr)?)),
            None => None,
        },
    })
}

fn lower_literal(expr: &Expr) -> CompilerResult<LiteralAnnotExpr> {
    let ExprKind::Literal(lit) = &expr.item else {
        return Err(error!(
            LoweringError::MismatchedNodes {
                expected: "literal".to_string(),
                found: expr.item.to_string(),
            },
            expr.span,
        ));
    };

    let ty = expr
        .ty
        .as_ref()
        .ok_or(error!(LoweringError::UnknownType, expr.span))?;

    macro_rules! num_pattern {
        ($value:expr, $value_ty:ty, $lit_kind:tt, $wrapper_struct:ty, $ty_kind:tt, $ty:ty, $err_kind:tt) => {{
            if $value <= <$ty>::MAX as $value_ty {
                Ok(LiteralAnnotExpr::$lit_kind(<$wrapper_struct>::$ty_kind(
                    $value as $ty,
                )))
            } else {
                Err(error!(
                    LoweringError::$err_kind {
                        amount: $value,
                        target: TypeKind::$ty_kind.to_string(),
                    },
                    expr.span,
                ))
            }
        }};
    }

    match lit {
        LiteralExpr::Int(value) => {
            let value = *value;

            match ty {
                TypeKind::UInt8 => num_pattern!(
                    value,
                    u64,
                    Int,
                    IntLiteralAnnotExpr,
                    UInt8,
                    u8,
                    IntegerOverflow
                ),
                TypeKind::UInt16 => num_pattern!(
                    value,
                    u64,
                    Int,
                    IntLiteralAnnotExpr,
                    UInt16,
                    u16,
                    IntegerOverflow
                ),
                TypeKind::UInt32 => num_pattern!(
                    value,
                    u64,
                    Int,
                    IntLiteralAnnotExpr,
                    UInt32,
                    u32,
                    IntegerOverflow
                ),
                TypeKind::UInt64 => num_pattern!(
                    value,
                    u64,
                    Int,
                    IntLiteralAnnotExpr,
                    UInt64,
                    u64,
                    IntegerOverflow
                ),
                TypeKind::Int8 => num_pattern!(
                    value,
                    u64,
                    Int,
                    IntLiteralAnnotExpr,
                    Int8,
                    i8,
                    IntegerOverflow
                ),
                TypeKind::Int16 => num_pattern!(
                    value,
                    u64,
                    Int,
                    IntLiteralAnnotExpr,
                    Int16,
                    i16,
                    IntegerOverflow
                ),
                TypeKind::Unit | TypeKind::Int32 => num_pattern!(
                    value,
                    u64,
                    Int,
                    IntLiteralAnnotExpr,
                    Int32,
                    i32,
                    IntegerOverflow
                ),
                TypeKind::Int64 => num_pattern!(
                    value,
                    u64,
                    Int,
                    IntLiteralAnnotExpr,
                    Int64,
                    i64,
                    IntegerOverflow
                ),

                TypeKind::Float32 => num_pattern!(
                    value,
                    u64,
                    Float,
                    FloatLiteralAnnotExpr,
                    Float32,
                    f32,
                    IntegerOverflow
                ),
                TypeKind::Float64 => num_pattern!(
                    value,
                    u64,
                    Float,
                    FloatLiteralAnnotExpr,
                    Float64,
                    f64,
                    IntegerOverflow
                ),

                TypeKind::Char => {
                    if value <= char::MAX as u64 {
                        Ok(LiteralAnnotExpr::Char(value as u8 as char))
                    } else {
                        Err(error!(
                            LoweringError::InvalidCast {
                                from: value.to_string(),
                                to: TypeKind::Char.to_string(),
                            },
                            expr.span
                        ))
                    }
                }

                _ => Err(error!(
                    LoweringError::InvalidTypeForIntLowering {
                        found: ty.clone(),
                    },
                    expr.span
                )),
            }
        }

        LiteralExpr::Float(value) => {
            let value = *value;

            match ty {
                TypeKind::Unit | TypeKind::Float32 => num_pattern!(
                    value,
                    f64,
                    Float,
                    FloatLiteralAnnotExpr,
                    Float32,
                    f32,
                    FloatOverflow
                ),
                TypeKind::Float64 => num_pattern!(
                    value,
                    f64,
                    Float,
                    FloatLiteralAnnotExpr,
                    Float64,
                    f64,
                    FloatOverflow
                ),

                _ => Err(error!(
                    LoweringError::InvalidTypeForFloatLowering {
                        found: ty.clone(),
                    },
                    expr.span,
                )),
            }
        }

        LiteralExpr::Bool(value) => Ok(LiteralAnnotExpr::Bool(*value)),
        LiteralExpr::Char(value) => Ok(LiteralAnnotExpr::Char(*value)),
        LiteralExpr::String(value) => Ok(LiteralAnnotExpr::String(value.to_string())),
        LiteralExpr::Unit => Ok(LiteralAnnotExpr::Unit),
    }
}

fn annotate_struct(struct_expr: StructExpr) -> CompilerResult<StructAnnotExpr> {
    Ok(StructAnnotExpr {
        symbol: annotate_symbol(struct_expr.symbol)?,
        fields: struct_expr
            .fields
            .into_iter()
            .map(|field| {
                Ok(StructFieldAnnotExpr {
                    symbol: annotate_symbol(field.symbol)?,
                    value: annotate_expr(field.value)?,
                })
            })
            .try_collect()?,
    })
}

fn annotate_tuple(tuple_expr: TupleExpr) -> CompilerResult<TupleAnnotExpr> {
    Ok(TupleAnnotExpr {
        elements: tuple_expr
            .elements
            .into_iter()
            .map(annotate_expr)
            .try_collect()?,
    })
}

fn annotate_unary(unary_expr: UnaryExpr) -> CompilerResult<UnaryAnnotExpr> {
    Ok(UnaryAnnotExpr {
        operator: unary_expr.operator,
        value: Box::new(annotate_expr(*unary_expr.value)?),
    })
}
