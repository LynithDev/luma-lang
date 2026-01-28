use luma_core::ast::*;

use crate::analyzer::{AnalyzerContext, AnalyzerStage, symbols::SymbolNamespace};

pub struct NameDeclaration;

impl AnalyzerStage for NameDeclaration {
    fn name(&self) -> String {
        String::from("name_declaration")
    }

    fn analyze(&mut self, ctx: &AnalyzerContext, input: &mut Ast) {
        self.traverse(ctx, input);
    }
}

impl AstVisitor<'_> for NameDeclaration {
    type Ctx = AnalyzerContext;

    fn visit_stmt(&mut self, ctx: &Self::Ctx, stmt: &mut Stmt) {
        let (namespace, symbol): (SymbolNamespace, &mut SymbolKind) = match &mut stmt.item {
            StmtKind::Var(var_decl) => {
                (SymbolNamespace::Value, &mut var_decl.symbol.item)
            },
            StmtKind::Struct(struct_decl) => {
                (SymbolNamespace::Type, &mut struct_decl.symbol.item)
            }
            StmtKind::Func(func_decl) => {
                (SymbolNamespace::Value, &mut func_decl.symbol.item)
            }
            _ => {
                return;
            }
        };

        let symbol_id = ctx.symbols.borrow_mut().declare(namespace, symbol.name().to_string());
        symbol.set_id(symbol_id);
    }

    fn visit_func_param(&mut self, ctx: &Self::Ctx, param: &mut FuncParam) {
        let symbol = &mut param.symbol.item;
        
        let symbol_id = ctx.symbols.borrow_mut().declare(
            SymbolNamespace::Value,
            symbol.name().to_string(),
        );

        symbol.set_id(symbol_id);
    }

    fn visit_struct_field_decl(&mut self, ctx: &Self::Ctx, struct_symbol: &Symbol, field: &mut StructFieldDecl) {
        let symbol = &mut field.symbol.item;
        
        let Some(struct_id) = struct_symbol.id() else {
            panic!("struct symbol must have an id when declaring its fields.");
        };

        let symbol_id = ctx.symbols.borrow_mut().declare(
            SymbolNamespace::StructField(struct_id),
            symbol.name().to_string(),
        );

        symbol.set_id(symbol_id);
    }
    

    fn enter_scope(&mut self, ctx: &Self::Ctx) {
        ctx.symbols.borrow_mut().enter_scope();
    }

    fn exit_scope(&mut self, ctx: &Self::Ctx) {
        ctx.symbols.borrow_mut().exit_scope();
    }
}