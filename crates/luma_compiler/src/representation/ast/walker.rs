use crate::{Type, TypeKind, ast::{Ast, Expr, ExprKind, FuncParam, Stmt, StmtKind, StructFieldDecl, StructFieldExpr, Symbol}};

#[allow(unused_variables)]
pub trait AstVisitor<'a> {
    type Ctx;

    fn visit_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {}
    fn leave_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {}

    fn visit_expr(&self, ctx: &mut Self::Ctx, expr: &mut Expr) {}
    fn leave_expr(&self, ctx: &mut Self::Ctx, expr: &mut Expr) {}

    fn visit_func_param(&self, ctx: &mut Self::Ctx, param: &mut FuncParam) {}
    fn visit_struct_field_decl(&self, ctx: &mut Self::Ctx, struct_symbol: &Symbol, field: &mut StructFieldDecl) {}
    fn visit_struct_field_expr(&self, ctx: &mut Self::Ctx, struct_symbol: &Symbol, field: &mut StructFieldExpr) {}
    fn visit_type(&self, ctx: &mut Self::Ctx, ty: &mut Type) {}
    
    fn enter_scope(&self, ctx: &mut Self::Ctx) {}
    fn exit_scope(&self, ctx: &mut Self::Ctx) {} 

    fn traverse(&self, ctx: &mut Self::Ctx, ast: &mut Ast) {
        for stmt in &mut ast.statements {
            self.walk_stmt(ctx, stmt);
        }
    }

    fn walk_expr(&self, ctx: &mut Self::Ctx, expr: &mut Expr) {
        self.visit_expr(ctx, expr);

        match &mut expr.item {
            ExprKind::Assign(assign_expr) => {
                self.walk_expr(ctx, &mut assign_expr.target);
                self.walk_expr(ctx, &mut assign_expr.value);
            },
            ExprKind::Binary(binary_expr) => {
                self.walk_expr(ctx, &mut binary_expr.left);
                self.walk_expr(ctx, &mut binary_expr.right);
            },
            ExprKind::Block(block_expr) => {
                self.enter_scope(ctx);

                for stmt in &mut block_expr.statements {
                    self.walk_stmt(ctx, stmt);
                }

                self.exit_scope(ctx);
            },
            ExprKind::Call(call_expr) => {
                self.walk_expr(ctx, &mut call_expr.callee);
                
                for arg in &mut call_expr.arguments {
                    self.walk_expr(ctx, arg);
                }
            },
            ExprKind::Get(get_expr) => {
                self.walk_expr(ctx, &mut get_expr.object);
            },
            ExprKind::Group(group) => {
                self.walk_expr(ctx, group);
            },
            ExprKind::If(if_expr) => {
                self.walk_expr(ctx, &mut if_expr.condition);
                self.walk_expr(ctx, &mut if_expr.then_branch);

                if let Some(else_branch) = &mut if_expr.else_branch {
                    self.walk_expr(ctx, else_branch);
                }
            },
            ExprKind::Struct(struct_expr) => {
                for field in &mut struct_expr.fields {
                    self.visit_struct_field_expr(ctx, &struct_expr.symbol, field);
                    self.walk_expr(ctx, &mut field.value);
                }
            },
            ExprKind::TupleLiteral(tuple_expr) => {
                for element in &mut tuple_expr.elements {
                    self.walk_expr(ctx, element);
                }
            },
            ExprKind::Unary(unary_expr) => {
                self.walk_expr(ctx, &mut unary_expr.value);
            },
            ExprKind::Ident(_)
            | ExprKind::Literal(_) => {
                // leaf nodes
            },
        }

        self.leave_expr(ctx, expr);
    }

    fn walk_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {
        self.visit_stmt(ctx, stmt);

        match &mut stmt.item {
            StmtKind::Expr(expr) => {
                self.walk_expr(ctx, expr);
            },
            StmtKind::Func(func_decl_stmt) => {
                self.enter_scope(ctx);

                for param in &mut func_decl_stmt.parameters {
                    self.walk_func_param(ctx, param);
                }

                if let Some(ty) = &mut func_decl_stmt.return_type {
                    self.walk_type(ctx, ty);
                }

                self.walk_expr(ctx, &mut func_decl_stmt.body);

                self.exit_scope(ctx);
            },
            StmtKind::Return(return_stmt) => {
                if let Some(value) = &mut return_stmt.value {
                    self.walk_expr(ctx, value);
                }
            },
            StmtKind::Struct(struct_decl_stmt) => {
                for field in &mut struct_decl_stmt.fields {
                    self.visit_struct_field_decl(ctx, &struct_decl_stmt.symbol, field);
                    self.walk_type(ctx, &mut field.ty);
                }
            },
            StmtKind::Var(var_decl_stmt) => {
                self.walk_expr(ctx, &mut var_decl_stmt.initializer);
            },
        }

        self.leave_stmt(ctx, stmt);
    }

    fn walk_func_param(&self, ctx: &mut Self::Ctx, param: &mut FuncParam) {
        self.visit_func_param(ctx, param);
        self.walk_type(ctx, &mut param.ty);

        if let Some(default_value) = &mut param.default_value {
            self.walk_expr(ctx, default_value);
        }
    }

    fn walk_type(&self, ctx: &mut Self::Ctx, ty: &mut Type) {
        self.visit_type(ctx, ty);

        match &mut ty.kind {
            TypeKind::Ptr(ty) => {
                self.walk_type(ctx, ty);
            },
            TypeKind::Tuple(types) => {
                for elem_type in types {
                    self.walk_type(ctx, elem_type);
                }
            },
            _ => {}
        }
    }
}