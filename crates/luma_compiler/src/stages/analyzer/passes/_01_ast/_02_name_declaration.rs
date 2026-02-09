use crate::stages::analyzer::{AnalyzerContext, AnalyzerPass, symbols::SymbolNamespace};
use crate::{ScopeId, SymbolId, Type, TypeKind, ast::*};

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

    fn leave_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut Stmt) {
        match &mut stmt.item {
            StmtKind::Var(var_decl) => {
                self.declare_symbol(
                    ctx,
                    stmt.scope_id.unwrap(),
                    &mut var_decl.symbol,
                    SymbolNamespace::Value,
                    var_decl.ty.clone(),
                );
            }
            StmtKind::Func(func_decl) => {
                self.declare_symbol(
                    ctx,
                    stmt.scope_id.unwrap(),
                    &mut func_decl.symbol,
                    SymbolNamespace::Value,
                    func_decl.return_type.clone(),
                );
            }
            StmtKind::Struct(struct_decl) => {
                // walk fields first
                // todo: struct fields decl
                // for (index, field) in struct_decl.fields.iter_mut().enumerate() {
                //     let symbol = &mut field.symbol.kind;

                //     // this should never occur, as we should only visit fields of declared structs
                //     let struct_id = struct_symbol.kind.id().unwrap();

                //     let current_scope = ctx.scopes.borrow().current_scope();
                //     let symbol_id = ctx.symbols.borrow_mut().declare(
                //         current_scope,
                //         SymbolNamespace::StructField(struct_id),
                //         symbol.name().to_string(),
                //         Some(field.ty.clone())
                //     );

                //     symbol.set_id(symbol_id);
                // }

                let ty = Some(Type::spanned(
                    struct_decl.symbol.span,
                    TypeKind::Named {
                        name: struct_decl.symbol.name().to_string(),
                        def_id: None,
                    },
                ));

                self.declare_symbol(ctx, stmt.scope_id.unwrap(), &mut struct_decl.symbol, SymbolNamespace::Type, ty);
            }
            _ => {},
        }
    }

    fn leave_func_param<'node>(&self, ctx: &mut Self::Ctx, _func: &'node FuncDeclStmt, param: &'node mut FuncParam) {
        self.declare_symbol(
            ctx,
            param.scope_id.unwrap(),
            &mut param.symbol,
            SymbolNamespace::Value,
            Some(param.ty.clone()),
        );
    }
}

impl NameDeclaration {
    fn declare_symbol(
        &self,
        ctx: &mut AnalyzerContext,
        scope_id: ScopeId,
        symbol: &mut Symbol,
        namespace: SymbolNamespace,
        declared_ty: Option<Type>,
    ) -> SymbolId {
        let symbol_id = ctx.symbols.borrow_mut().declare(
            scope_id,
            namespace,
            symbol.name().to_string(),
            declared_ty,
        );

        symbol.set_id(symbol_id);
        symbol_id
    }
}
