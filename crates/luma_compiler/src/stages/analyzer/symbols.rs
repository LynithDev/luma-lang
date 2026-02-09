use std::collections::HashMap;

use crate::{ScopeId, SymbolId, Type, stages::analyzer::scopes::ScopeManager};

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<SymbolEntry>,
    lookup_map: HashMap<ScopeId, HashMap<SymbolNamespace, HashMap<String, SymbolId>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolNamespace {
    ControlFlow,
    Type,
    Value,
    // /// field symbols within a struct
    // /// usize - index of the field within the struct
    // StructField(usize),
}

#[derive(Debug)]
pub struct SymbolEntry {
    pub name: String,
    pub namespace: SymbolNamespace,
    pub scope_id: ScopeId,
    pub declared_ty: Option<Type>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: Vec::new(),
            lookup_map: HashMap::new(),
        }
    }

    pub fn declare(
        &mut self,
        scope_id: ScopeId,
        namespace: SymbolNamespace,
        name: String,
        declared_ty: Option<Type>,
    ) -> SymbolId {
        let id = self.symbols.len();

        self.symbols.push(SymbolEntry {
            name: name.clone(),
            namespace,
            scope_id,
            declared_ty,
        });

        self.lookup_map
            .entry(scope_id)
            .or_default()
            .entry(namespace)
            .or_default()
            .insert(name.clone(), id);

        id
    }

    pub fn lookup(
        &self,
        scopes: &ScopeManager,
        namespace: SymbolNamespace,
        mut scope: ScopeId,
        name: &str,
    ) -> Option<SymbolId> {
        while let Some(scope_map) = self.lookup_map.get(&scope) {
            if let Some(ns_map) = scope_map.get(&namespace)
                && let Some(&id) = ns_map.get(name)
            {
                return Some(id);
            }

            scope = match scopes.parent(scope) {
                Some(p) => p,
                None => break,
            };
        }
        None
    }

    pub fn enter_scope(&mut self, scope_id: ScopeId) {
        self.lookup_map.entry(scope_id).or_default();
    }

    pub fn exit_scope(&mut self, _scope: ScopeId) {
        // no-op
    }

    pub fn get_symbol(&self, id: SymbolId) -> Option<&SymbolEntry> {
        self.symbols.get(id)
    }
}
