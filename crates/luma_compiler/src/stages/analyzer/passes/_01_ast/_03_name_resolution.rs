use luma_diagnostic::error;

use crate::ast::*;

use crate::stages::analyzer::{AnalyzerContext, AnalyzerPass, error::AnalyzerErrorKind, symbols::SymbolNamespace};

pub struct NameResolution;

impl AnalyzerPass<Ast> for NameResolution {
    fn name(&self) -> String {
        String::from("name_resolution")
    }

    fn analyze(&self, ctx: &mut AnalyzerContext, input: &mut Ast) {
        self.traverse(ctx, input);
    }
}

impl AstVisitor<'_> for NameResolution {
    type Ctx = AnalyzerContext;

    // here we resolve identifiers to their declared symbols
    fn visit_expr(&self, ctx: &mut Self::Ctx, expr: &mut Expr) {
        match &mut expr.item {
            ExprKind::Ident(ident_expr) => {
                let symbol = &mut ident_expr.symbol;
                let scope_manager = ctx.scopes.borrow();

                // lookup the symbol in value namespace, 
                // as identifiers refer to variables
                let Some(resolved_id) = ctx.symbols.borrow_mut().lookup(
                    &scope_manager,
                    SymbolNamespace::Value, 
                    scope_manager.current_scope(),
                    symbol.name()
                ) else {
                    ctx.error(error!(
                        AnalyzerErrorKind::UnresolvedIdentifier(symbol.name().to_string()), 
                        expr.span,
                    ));

                    return;
                };
                
                // if the symbol was found, set the id, else report an error
                symbol.set_id(resolved_id);
            }
            ExprKind::Struct(struct_expr) => {
                let symbol = &mut struct_expr.symbol;
                let scope_manager = ctx.scopes.borrow();

                // lookup the symbol in type namespace, 
                // as identifiers refer to types
                let Some(resolved_id) = ctx.symbols.borrow_mut().lookup(
                    &scope_manager,
                    SymbolNamespace::Type,
                    scope_manager.current_scope(), 
                    symbol.kind.name()
                ) else {
                    ctx.error(error!(
                        AnalyzerErrorKind::UnresolvedType(symbol.name().to_string()), 
                        expr.span,
                    ));
                    return;
                };
                
                // if the symbol was found, set the id, else report an error
                symbol.set_id(resolved_id);
            }
            _ => {}
        }
    }

    fn visit_struct_field_expr(&self, ctx: &mut Self::Ctx, struct_symbol: &Symbol, field: &mut StructFieldExpr) {
        let field_symbol = &mut field.symbol.kind;

        // lookup the symbol in type namespace, 
        // as identifiers refer to types
        let Some(resolved_struct_id) = struct_symbol.id() else {
            ctx.error(error!(
                AnalyzerErrorKind::UnresolvedType(struct_symbol.name().to_string()), 
                struct_symbol.span,
            ));
            return;
        };

        let scope_manager = ctx.scopes.borrow();

        let Some(resolved_field_id) = ctx.symbols.borrow_mut().lookup(
            &scope_manager,
            SymbolNamespace::StructField(resolved_struct_id),
            scope_manager.current_scope(),
            field_symbol.name()
        ) else {
            ctx.error(error!(
                AnalyzerErrorKind::UnresolvedStructField {
                    struct_name: struct_symbol.name().to_string(), 
                    field_name: field_symbol.name().to_string()
                }, 
                field.symbol.span,
            ));
            return;
        };

        field_symbol.set_id(resolved_field_id);
    }


    fn enter_scope(&self, ctx: &mut Self::Ctx) {
        ctx.symbols.borrow_mut().enter_scope();
    }

    fn exit_scope(&self, ctx: &mut Self::Ctx) {
        let scope_id = ctx.scopes.borrow().current_scope();
        ctx.symbols.borrow_mut().exit_scope(scope_id);
    }
}
