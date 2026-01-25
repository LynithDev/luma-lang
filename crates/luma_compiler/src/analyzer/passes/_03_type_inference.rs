use luma_core::ast::*;

use crate::analyzer::{AnalyzerContext, AnalyzerStage};

pub struct TypeInference;

impl AnalyzerStage for TypeInference {
    fn name(&self) -> String {
        String::from("type_inference")
    }

    fn analyze(&mut self, _ctx: &AnalyzerContext, input: &mut Ast) {
        
    }
}

impl AstVisitor for TypeInference {
    type Ctx = AnalyzerContext;

    fn visit_stmt(&mut self, ctx: &Self::Ctx, stmt: &mut Stmt) {
        match &mut stmt.item {
            StmtKind::Var(var_decl) => {
                // var_decl.ty = 
            },
            StmtKind::Func(func_decl) => {
                
            },
            _ => {}
        }
    }

    fn visit_expr(&mut self, ctx: &Self::Ctx, expr: &mut Expr) {
        match &mut expr.item {
            ExprKind::Literal(lit) => {
                
            },
            ExprKind::Binary(bin_expr) => {
                // Infer type based on binary expression operands and operator
            },
            _ => {}
        }
    }
}

impl TypeInference {
    fn infer_type(&self, expr: &Expr) -> Option<TypeKind> {
        match &expr.item {
            ExprKind::Literal(lit) => Some(self.infer_type_from_literal(lit)),
            ExprKind::Binary(bin_expr) => {
                None
            },
            _ => None,
        }
    }

    fn infer_type_from_literal(&self, lit: &LiteralExpr) -> TypeKind {
        match lit {
            LiteralExpr::Int(num) => {
                if *num <= u32::MAX as u64 {
                    TypeKind::UInt32
                } else {
                    TypeKind::UInt64
                }
            },
            LiteralExpr::Float(num) => {
                if *num <= f32::MAX as f64 {
                    TypeKind::Float32
                } else {
                    TypeKind::Float64
                }
            },
            LiteralExpr::Bool(_) => TypeKind::Bool,
            LiteralExpr::String(_) => TypeKind::String,
            LiteralExpr::Char(_) => TypeKind::Char,
        }
    }
}