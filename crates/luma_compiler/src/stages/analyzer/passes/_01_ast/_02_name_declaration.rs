use crate::stages::analyzer::{AnalyzerContext, AnalyzerPass, symbols::SymbolNamespace};
use crate::ast::*;

pub struct NameDeclaration;

impl AnalyzerPass<Ast> for NameDeclaration {
    fn name(&self) -> String {
        "name_declaration".to_string()
    }

    fn analyze(&self, ctx: &mut AnalyzerContext, input: &mut Ast) {
        self.traverse(ctx, input);
    }
}

impl AstVisitor<'_> for NameDeclaration {
    type Ctx = AnalyzerContext;

    fn visit_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {
        let (namespace, symbol) = match &mut stmt.item {
            StmtKind::Var(var_decl) => (SymbolNamespace::Value, &mut var_decl.symbol.kind),
            StmtKind::Func(func_decl) => (SymbolNamespace::Value, &mut func_decl.symbol.kind),
            StmtKind::Struct(struct_decl) => (SymbolNamespace::Type, &mut struct_decl.symbol.kind),
            _ => return,
        };

        let current_scope = ctx.scopes.borrow().current_scope();
        let symbol_id =
            ctx.symbols
                .borrow_mut()
                .declare(current_scope, namespace, symbol.name().to_string());
            
        symbol.set_id(symbol_id);
    }

    fn visit_func_param(&self, ctx: &mut Self::Ctx, param: &mut FuncParam) {
        let symbol = &mut param.symbol.kind;
        
        let current_scope = ctx.scopes.borrow().current_scope();
        let symbol_id = ctx.symbols.borrow_mut().declare(
            current_scope,
            SymbolNamespace::Value,
            symbol.name().to_string(),
        );

        symbol.set_id(symbol_id);
    }

    fn visit_struct_field_decl(
        &self,
        ctx: &mut Self::Ctx,
        struct_symbol: &Symbol,
        field: &mut StructFieldDecl,
    ) {
        let symbol = &mut field.symbol.kind;

        // this should never occur, as we should only visit fields of declared structs
        let struct_id = struct_symbol.kind.id().unwrap();
        
        let current_scope = ctx.scopes.borrow().current_scope();
        let symbol_id = ctx.symbols.borrow_mut().declare(
            current_scope,
            SymbolNamespace::StructField(struct_id),
            symbol.name().to_string(),
        );

        symbol.set_id(symbol_id);
    }

    fn enter_scope(&self, ctx: &mut Self::Ctx) {
        ctx.scopes.borrow_mut().enter_scope();
        ctx.symbols.borrow_mut().enter_scope();
    }

    fn exit_scope(&self, ctx: &mut Self::Ctx) {
        let scope = ctx.scopes.borrow_mut().current_scope();

        ctx.symbols.borrow_mut().exit_scope(scope);
        ctx.scopes.borrow_mut().exit_scope();
    }
}
