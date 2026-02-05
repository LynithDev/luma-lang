use luma_diagnostic::{CompilerResult, LumaError};

use crate::{
    CompilerContext, CompilerStage, TypeKind, aast::*, ast::*, stages::lowering::error::LoweringErrorKind
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
            let aast = match AnnotatedAst::try_from(ast) {
                Ok(aast) => aast,
                Err(err) => {
                    ctx.errors.borrow_mut().push(err);
                    return Vec::new();
                }
            };

            aasts.push(aast);
        }

        aasts
    }
}

impl TryFrom<Ast> for AnnotatedAst {
    type Error = LumaError;

    fn try_from(value: Ast) -> Result<Self, Self::Error> {
        Ok(Self {
            statements: value
                .statements
                .into_iter()
                .map(|stmt| stmt.try_into())
                .try_collect()?,
            span: value.span,
        })
    }
}

impl TryFrom<Symbol> for AnnotSymbol {
    type Error = LumaError;

    fn try_from(value: Symbol) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name().to_string(),
            id: value
                .id()
                .ok_or(LumaError::new(LoweringErrorKind::MissingSymbolId))?,
            span: value.span,
        })
    }
}

impl TryFrom<Stmt> for AnnotStmt {
    type Error = LumaError;

    fn try_from(value: Stmt) -> Result<Self, Self::Error> {
        Ok(AnnotStmt {
            item: match value.item {
                StmtKind::Expr(expr) => AnnotStmtKind::Expr(expr.try_into()?),
                StmtKind::Func(func_decl_stmt) => AnnotStmtKind::Func(func_decl_stmt.try_into()?),
                StmtKind::Return(return_stmt) => AnnotStmtKind::Return(return_stmt.try_into()?),
                StmtKind::Struct(struct_decl_stmt) => {
                    AnnotStmtKind::Struct(struct_decl_stmt.try_into()?)
                }
                StmtKind::Var(var_decl_stmt) => AnnotStmtKind::Var(var_decl_stmt.try_into()?),
            },
            scope_id: value
                .scope_id
                .ok_or(LumaError::new(LoweringErrorKind::MissingScopeId))?,
            span: value.span,
        })
    }
}

impl TryFrom<FuncDeclStmt> for FuncDeclAnnotStmt {
    type Error = LumaError;

    fn try_from(value: FuncDeclStmt) -> Result<Self, Self::Error> {
        Ok(Self {
            visibility: value.visibility,
            symbol: value.symbol.try_into()?,
            parameters: value
                .parameters
                .into_iter()
                .map(|param| param.try_map_inner())
                .try_collect()?,
            body: value.body.try_into()?,
            return_type: value
                .return_type
                .ok_or(LumaError::new(LoweringErrorKind::UnknownType))?,
        })
    }
}

impl TryFrom<FuncParam> for AnnotFuncParam {
    type Error = LumaError;

    fn try_from(value: FuncParam) -> Result<Self, Self::Error> {
        Ok(Self {
            symbol: value.symbol.try_into()?,
            ty: value.ty,
            default_value: value
                .default_value
                .map(|value| value.try_into())
                .transpose()?,
        })
    }
}

impl TryFrom<ReturnStmt> for ReturnAnnotStmt {
    type Error = LumaError;

    fn try_from(value: ReturnStmt) -> Result<Self, Self::Error> {
        Ok(Self {
            value: value.value.map(|value| value.try_into()).transpose()?,
        })
    }
}

impl TryFrom<StructDeclStmt> for StructDeclAnnotStmt {
    type Error = LumaError;

    fn try_from(value: StructDeclStmt) -> Result<Self, Self::Error> {
        Ok(Self {
            visibility: value.visibility,
            symbol: value.symbol.try_into()?,
            fields: value
                .fields
                .into_iter()
                .map(|field| field.try_map_inner())
                .try_collect()?,
        })
    }
}

impl TryFrom<StructFieldDecl> for StructFieldAnnotDecl {
    type Error = LumaError;

    fn try_from(value: StructFieldDecl) -> Result<Self, Self::Error> {
        Ok(Self {
            visibility: value.visibility,
            symbol: value.symbol.try_into()?,
            ty: value.ty,
        })
    }
}

impl TryFrom<VarDeclStmt> for VarDeclAnnotStmt {
    type Error = LumaError;

    fn try_from(value: VarDeclStmt) -> Result<Self, Self::Error> {
        Ok(Self {
            visibility: value.visibility,
            symbol: value.symbol.try_into()?,
            ty: value
                .ty
                .ok_or(LumaError::new(LoweringErrorKind::UnknownType))?,
            initializer: value.initializer.try_into()?,
        })
    }
}

impl TryFrom<Expr> for AnnotExpr {
    type Error = LumaError;

    fn try_from(value: Expr) -> Result<Self, Self::Error> {
        Ok(AnnotExpr {
            item: match value.item {
                ExprKind::Assign(assign_expr) => AnnotExprKind::Assign(assign_expr.try_into()?),
                ExprKind::Binary(binary_expr) => AnnotExprKind::Binary(binary_expr.try_into()?),
                ExprKind::Block(block_expr) => AnnotExprKind::Block(block_expr.try_into()?),
                ExprKind::Call(call_expr) => AnnotExprKind::Call(call_expr.try_into()?),
                ExprKind::Get(get_expr) => AnnotExprKind::Get(get_expr.try_into()?),
                ExprKind::Group(expr) => AnnotExprKind::Group(Box::new((*expr).try_into()?)),
                ExprKind::Ident(ident_expr) => AnnotExprKind::Ident(ident_expr.try_into()?),
                ExprKind::If(if_expr) => AnnotExprKind::If(if_expr.try_into()?),
                ExprKind::Literal(_) => AnnotExprKind::Literal(lower_literal(&value)?),
                ExprKind::Struct(struct_expr) => AnnotExprKind::Struct(struct_expr.try_into()?),
                ExprKind::TupleLiteral(tuple_expr) => {
                    AnnotExprKind::TupleLiteral(tuple_expr.try_into()?)
                }
                ExprKind::Unary(unary_expr) => AnnotExprKind::Unary(unary_expr.try_into()?),
            },
            ty: value
                .ty
                .ok_or(LumaError::new(LoweringErrorKind::UnknownType))?,
            scope_id: value
                .scope_id
                .ok_or(LumaError::new(LoweringErrorKind::MissingScopeId))?,
            span: value.span,
        })
    }
}

impl TryFrom<AssignExpr> for AssignAnnotExpr {
    type Error = LumaError;
    fn try_from(value: AssignExpr) -> Result<Self, Self::Error> {
        Ok(Self {
            target: Box::new((*value.target).try_into()?),
            operator: value.operator,
            value: Box::new((*value.value).try_into()?),
        })
    }
}

impl TryFrom<BinaryExpr> for BinaryAnnotExpr {
    type Error = LumaError;
    fn try_from(value: BinaryExpr) -> Result<Self, Self::Error> {
        Ok(Self {
            left: Box::new((*value.left).try_into()?),
            operator: value.operator,
            right: Box::new((*value.right).try_into()?),
        })
    }
}

impl TryFrom<BlockExpr> for BlockAnnotExpr {
    type Error = LumaError;
    fn try_from(value: BlockExpr) -> Result<Self, Self::Error> {
        Ok(Self {
            statements: value
                .statements
                .into_iter()
                .map(|stmt| stmt.try_into())
                .try_collect()?,
        })
    }
}

impl TryFrom<CallExpr> for CallAnnotExpr {
    type Error = LumaError;
    fn try_from(value: CallExpr) -> Result<Self, Self::Error> {
        Ok(Self {
            callee: Box::new((*value.callee).try_into()?),
            arguments: value
                .arguments
                .into_iter()
                .map(|arg| arg.try_into())
                .try_collect()?,
        })
    }
}

impl TryFrom<GetExpr> for GetAnnotExpr {
    type Error = LumaError;
    fn try_from(value: GetExpr) -> Result<Self, Self::Error> {
        Ok(Self {
            object: Box::new((*value.object).try_into()?),
            property: value.property.try_into()?,
        })
    }
}

impl TryFrom<IdentExpr> for IdentAnnotExpr {
    type Error = LumaError;
    fn try_from(value: IdentExpr) -> Result<Self, Self::Error> {
        let name = value.symbol.name().to_string();
        let id = value
            .symbol
            .id()
            .ok_or(LumaError::new(LoweringErrorKind::MissingSymbolId))?;
        Ok(Self {
            symbol: AnnotSymbol {
                name,
                id,
                span: Default::default(), // You may want to pass the correct span
            },
        })
    }
}

impl TryFrom<IfExpr> for IfAnnotExpr {
    type Error = LumaError;
    fn try_from(value: IfExpr) -> Result<Self, Self::Error> {
        Ok(Self {
            condition: Box::new((*value.condition).try_into()?),
            then_branch: Box::new((*value.then_branch).try_into()?),
            else_branch: match value.else_branch {
                Some(else_expr) => Some(Box::new((*else_expr).try_into()?)),
                None => None,
            },
        })
    }
}

fn lower_literal(expr: &Expr) -> CompilerResult<LiteralAnnotExpr> {
    let ExprKind::Literal(lit) = &expr.item else {
        return Err(LumaError::new(LoweringErrorKind::InvalidLiteralConversion(
            expr.item.to_string(),
        )));
    };

    let ty = expr.ty.as_ref().ok_or(LumaError::new(
        LoweringErrorKind::UnknownType
    ))?;

    macro_rules! num_pattern {
        ($value:expr, $value_ty:ty, $lit_kind:tt, $wrapper_struct:ty, $ty_kind:tt, $ty:ty, $err_kind:tt) => {{
            if $value <= <$ty>::MAX as $value_ty {
                Ok(LiteralAnnotExpr::$lit_kind(<$wrapper_struct>::$ty_kind($value as $ty)))
            } else {
                Err(LumaError::new(LoweringErrorKind::$err_kind {
                    amount: $value,
                    target: TypeKind::$ty_kind.to_string(),
                }))
            }
        }};
    }

    match lit {
        LiteralExpr::Int(value) => {
            let value = *value;

            match ty {
                TypeKind::UInt8 => num_pattern!(value, u64, Int, IntLiteralAnnotExpr, UInt8, u8, IntegerOverflow),
                TypeKind::UInt16 => num_pattern!(value, u64, Int, IntLiteralAnnotExpr, UInt16, u16, IntegerOverflow),
                TypeKind::UInt32 => num_pattern!(value, u64, Int, IntLiteralAnnotExpr, UInt32, u32, IntegerOverflow),
                TypeKind::UInt64 => num_pattern!(value, u64, Int, IntLiteralAnnotExpr, UInt64, u64, IntegerOverflow),
                TypeKind::Int8 => num_pattern!(value, u64, Int, IntLiteralAnnotExpr, Int8, i8, IntegerOverflow),
                TypeKind::Int16 => num_pattern!(value, u64, Int, IntLiteralAnnotExpr, Int16, i16, IntegerOverflow),
                TypeKind::Int32 => num_pattern!(value, u64, Int, IntLiteralAnnotExpr, Int32, i32, IntegerOverflow),
                TypeKind::Int64 => num_pattern!(value, u64, Int, IntLiteralAnnotExpr, Int64, i64, IntegerOverflow),

                TypeKind::Float32 => num_pattern!(value, u64, Float, FloatLiteralAnnotExpr, Float32, f32, IntegerOverflow),
                TypeKind::Float64 => num_pattern!(value, u64, Float, FloatLiteralAnnotExpr, Float64, f64, IntegerOverflow),

                TypeKind::Char => {
                    if value <= char::MAX as u64 {
                        Ok(LiteralAnnotExpr::Char(value as u8 as char))
                    } else {
                        Err(LumaError::new(LoweringErrorKind::InvalidCast {
                            from: value.to_string(),
                            to: TypeKind::Char.to_string(),
                        }))
                    }
                }

                _ => Err(LumaError::new(LoweringErrorKind::InvalidLiteralConversion(
                    expr.item.to_string(),
                ))),
            }
        }

        LiteralExpr::Float(value) => {
            let value = *value;

            match ty {
                TypeKind::Float32 => num_pattern!(value, f64, Float, FloatLiteralAnnotExpr, Float32, f32, FloatOverflow),
                TypeKind::Float64 => num_pattern!(value, f64, Float, FloatLiteralAnnotExpr, Float64, f64, FloatOverflow),

                _ => Err(LumaError::new(LoweringErrorKind::InvalidLiteralConversion(
                    expr.item.to_string(),
                ))),
            }
        }

        LiteralExpr::Bool(value) => Ok(LiteralAnnotExpr::Bool(*value)),
        LiteralExpr::Char(value) => Ok(LiteralAnnotExpr::Char(*value)),
        LiteralExpr::String(value) => Ok(LiteralAnnotExpr::String(value.to_string())),
        LiteralExpr::Unit => Ok(LiteralAnnotExpr::Unit),
    }
}

impl TryFrom<StructExpr> for StructAnnotExpr {
    type Error = LumaError;
    fn try_from(value: StructExpr) -> Result<Self, Self::Error> {
        Ok(Self {
            symbol: value.symbol.try_into()?,
            fields: value
                .fields
                .into_iter()
                .map(|field| field.try_into())
                .try_collect()?,
        })
    }
}

impl TryFrom<StructFieldExpr> for StructFieldAnnotExpr {
    type Error = LumaError;
    fn try_from(value: StructFieldExpr) -> Result<Self, Self::Error> {
        Ok(Self {
            symbol: value.symbol.try_into()?,
            value: value.value.try_into()?,
        })
    }
}

impl TryFrom<TupleExpr> for TupleAnnotExpr {
    type Error = LumaError;
    fn try_from(value: TupleExpr) -> Result<Self, Self::Error> {
        Ok(Self {
            elements: value
                .elements
                .into_iter()
                .map(|el| el.try_into())
                .try_collect()?,
        })
    }
}

impl TryFrom<UnaryExpr> for UnaryAnnotExpr {
    type Error = LumaError;
    fn try_from(value: UnaryExpr) -> Result<Self, Self::Error> {
        Ok(Self {
            operator: value.operator,
            value: Box::new((*value.value).try_into()?),
        })
    }
}
