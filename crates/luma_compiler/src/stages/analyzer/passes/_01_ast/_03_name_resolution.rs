use luma_diagnostic::{context, error};

use crate::ast::*;

use crate::stages::analyzer::AnalyzerErrorContext;
use crate::stages::analyzer::{AnalyzerContext, AnalyzerPass, AnalyzerError, symbols::SymbolNamespace};

pub struct NameResolution;

impl AnalyzerPass<Ast> for NameResolution {
    fn name(&self) -> String {
        String::from("name_resolution")
    }

    fn analyze(&self, ctx: &mut AnalyzerContext, input: &mut Ast) {
        self.traverse(ctx, input);
    }

    fn continue_after_error(&self) -> bool {
        false
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
                    expr.scope_id.unwrap(),
                    symbol.name()
                ) else {
                    ctx.diagnostic(error!(
                        AnalyzerError::UnresolvedIdentifier {
                            identifier: symbol.name().to_string(),
                        }, 
                        [
                            context!(AnalyzerErrorContext::ScopeContext { 
                                scope_id: expr.scope_id.unwrap(),
                            })
                        ],
                        expr.span,
                    ));

                    return;
                };
                
                // if the symbol was found, set the id, else report an error
                symbol.set_id(resolved_id);
            }
            ExprKind::Struct(struct_expr) => {
                let struct_symbol = &mut struct_expr.symbol;
                let scope_manager = ctx.scopes.borrow();

                // lookup the symbol in type namespace, 
                // as identifiers refer to types
                let Some(resolved_id) = ctx.symbols.borrow_mut().lookup(
                    &scope_manager,
                    SymbolNamespace::Type,
                    expr.scope_id.unwrap(), 
                    struct_symbol.kind.name()
                ) else {
                    ctx.diagnostic(error!(
                        AnalyzerError::UnresolvedType {
                            name: struct_symbol.name().to_string(),
                        }, 
                        expr.span,
                    ));
                    return;
                };

                // now walk through fields and resolve them as well, using the resolved struct id to lookup the field symbols
                // todo: struct fields name resoltun
                // for (index, field) in struct_expr.fields.iter_mut().enumerate() {
                //     let field_symbol = &mut field.symbol.kind;

                //     let scope_manager = ctx.scopes.borrow();

                //     let Some(resolved_field_id) = ctx.symbols.borrow_mut().lookup(
                //         &scope_manager,
                //         SymbolNamespace::StructField(index),
                //         expr.scope_id.unwrap(),
                //         field_symbol.name()
                //     ) else {
                //         ctx.diagnostic(error!(
                //             AnalyzerError::UnresolvedStructField {
                //                 struct_name: struct_symbol.name().to_string(), 
                //                 field_name: field_symbol.name().to_string()
                //             }, 
                //             field.symbol.span,
                //         ));
                //         return;
                //     };

                //     field_symbol.set_id(resolved_field_id);
                // }
                
                // if the symbol was found, set the id, else report an error
                struct_symbol.set_id(resolved_id);
            }
            _ => {}
        }
    }

    fn enter_scope(&self, ctx: &mut Self::Ctx, entering_scope_id: Option<crate::ScopeId>) {
        ctx.symbols.borrow_mut().enter_scope(entering_scope_id.unwrap());
    }

    fn exit_scope(&self, ctx: &mut Self::Ctx, leaving_scope_id: Option<crate::ScopeId>) {
        ctx.symbols.borrow_mut().exit_scope(leaving_scope_id.unwrap());
    }
}
