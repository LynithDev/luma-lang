use luma_diagnostic::CompilerResult;

use crate::{Type, TypeKind, aast::*};

#[allow(unused_variables)]
pub trait AnnotAstVisitor<'a> {
    type Ctx;

    fn try_visit_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut AnnotStmt) -> CompilerResult<()> {
        Ok(())
    }
    
    fn try_leave_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut AnnotStmt) -> CompilerResult<()> {
        Ok(())
    }

    fn try_visit_expr(&self, ctx: &mut Self::Ctx, expr: &mut AnnotExpr) -> CompilerResult<()> {
        Ok(())
    }

    fn try_leave_expr(&self, ctx: &mut Self::Ctx, expr: &mut AnnotExpr) -> CompilerResult<()> {
        Ok(())
    }

    fn try_visit_type(&self, ctx: &mut Self::Ctx, ty: &mut Type) -> CompilerResult<()> {
        Ok(())
    }
    
    fn try_leave_type(&self, ctx: &mut Self::Ctx, ty: &mut Type) -> CompilerResult<()> {
        Ok(())
    }
    
    fn enter_scope(&self, ctx: &mut Self::Ctx) {}
    fn exit_scope(&self, ctx: &mut Self::Ctx) {}

    fn traverse(&self, ctx: &mut Self::Ctx, ast: &mut AnnotatedAst) -> CompilerResult<()> {
        for stmt in &mut ast.statements {
            self.walk_stmt(ctx, stmt)?;
        }

        Ok(())
    }

    fn walk_expr(&self, ctx: &mut Self::Ctx, expr: &mut AnnotExpr) -> CompilerResult<()> {
        self.try_visit_expr(ctx, expr)?;

        match &mut expr.item {
            AnnotExprKind::Assign(assign_expr) => {
                self.walk_expr(ctx, &mut assign_expr.target)?;
                self.walk_expr(ctx, &mut assign_expr.value)?;
            },
            AnnotExprKind::Binary(binary_expr) => {
                self.walk_expr(ctx, &mut binary_expr.left)?;
                self.walk_expr(ctx, &mut binary_expr.right)?;
            },
            AnnotExprKind::Block(block_expr) => {
                self.enter_scope(ctx);

                for stmt in &mut block_expr.statements {
                    self.walk_stmt(ctx, stmt)?;
                }

                self.exit_scope(ctx);
            },
            AnnotExprKind::Call(call_expr) => {
                self.walk_expr(ctx, &mut call_expr.callee)?;
                
                for arg in &mut call_expr.arguments {
                    self.walk_expr(ctx, arg)?;
                }
            },
            AnnotExprKind::Get(get_expr) => {
                self.walk_expr(ctx, &mut get_expr.object)?;
            },
            AnnotExprKind::Group(group) => {
                self.walk_expr(ctx, group)?;
            },
            AnnotExprKind::If(if_expr) => {
                self.walk_expr(ctx, &mut if_expr.condition)?;
                self.walk_expr(ctx, &mut if_expr.then_branch)?;

                if let Some(else_branch) = &mut if_expr.else_branch {
                    self.walk_expr(ctx, else_branch)?;
                }
            },
            AnnotExprKind::Struct(struct_expr) => {
                for field in &mut struct_expr.fields {
                    self.walk_expr(ctx, &mut field.value)?;
                }
            },
            AnnotExprKind::TupleLiteral(tuple_expr) => {
                for element in &mut tuple_expr.elements {
                    self.walk_expr(ctx, element)?;
                }
            },
            AnnotExprKind::Unary(unary_expr) => {
                self.walk_expr(ctx, &mut unary_expr.value)?;
            },
            AnnotExprKind::Ident(_)
            | AnnotExprKind::Literal(_) => {
                // leaf nodes
            },
        }

        self.try_leave_expr(ctx, expr)
    }

    fn walk_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut AnnotStmt) -> CompilerResult<()> {
        self.try_visit_stmt(ctx, stmt)?;

        match &mut stmt.item {
            AnnotStmtKind::Expr(expr) => {
                self.walk_expr(ctx, expr)?;
            },
            AnnotStmtKind::Func(func_decl_stmt) => {
                self.enter_scope(ctx);

                for param in &mut func_decl_stmt.parameters {
                    self.walk_type(ctx, &mut param.ty)?;
        
                    if let Some(default_value) = &mut param.default_value {
                        self.walk_expr(ctx, default_value)?;
                    }
                }

                self.walk_type(ctx, &mut func_decl_stmt.return_type)?;

                self.walk_expr(ctx, &mut func_decl_stmt.body)?;

                self.exit_scope(ctx);
            },
            AnnotStmtKind::Return(return_stmt) => {
                if let Some(value) = &mut return_stmt.value {
                    self.walk_expr(ctx, value)?;
                }
            },
            AnnotStmtKind::Struct(struct_decl_stmt) => {
                for field in &mut struct_decl_stmt.fields {
                    self.walk_type(ctx, &mut field.ty)?;
                }
            },
            AnnotStmtKind::Var(var_decl_stmt) => {
                self.walk_expr(ctx, &mut var_decl_stmt.initializer)?;
            },
        }

        self.try_leave_stmt(ctx, stmt)
    }

    fn walk_type(&self, ctx: &mut Self::Ctx, ty: &mut Type) -> CompilerResult<()> {
        self.try_visit_type(ctx, ty)?;

        match &mut ty.kind {
            TypeKind::Ptr(ty) => {
                self.walk_type(ctx, ty)?;
            },
            TypeKind::Tuple(types) => {
                for elem_type in types {
                    self.walk_type(ctx, elem_type)?;
                }
            },
            _ => {}
        }
        
        self.try_leave_type(ctx, ty)
    }
}