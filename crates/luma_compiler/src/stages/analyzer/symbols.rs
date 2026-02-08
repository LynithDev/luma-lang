use std::collections::HashMap;

use crate::{ScopeId, SymbolId, Type, stages::analyzer::scopes::ScopeManager};

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<SymbolEntry>,
    /// (namespace + name) -> symbol id
    lookup_map: HashMap<(SymbolNamespace, ScopeId, String), SymbolId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolNamespace {
    ControlFlow,
    Type,
    Value,
    /// field symbols within a struct
    /// usize - index of the field within the struct
    StructField(usize),
}

#[derive(Debug)]
pub struct SymbolEntry {
    pub name: String,
    pub namespace: SymbolNamespace,
    pub scope_id: ScopeId,
    pub declared_ty: Option<Type>,
    pub shadowed: Option<SymbolId>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: Vec::new(),
            lookup_map: HashMap::new(),
        }
    }

    pub fn declare(&mut self, scope_id: ScopeId, namespace: SymbolNamespace, name: String, declared_ty: Option<Type>) -> SymbolId {
        let shadowed = self.lookup_map.get(&(namespace, scope_id, name.clone())).cloned();

        let id = self.symbols.len();
        self.symbols.push(SymbolEntry {
            name: name.clone(),
            namespace,
            scope_id,
            declared_ty,
            shadowed,
        });

        self.lookup_map.insert((namespace, scope_id, name), id);

        id
    }

    pub fn lookup(
        &self,
        scopes: &ScopeManager,
        namespace: SymbolNamespace, 
        mut scope: ScopeId,
        name: &str,
    ) -> Option<SymbolId> {
        loop {
            if let Some(id) = self.lookup_map.get(&(namespace, scope, name.to_string())) {
                return Some(*id);
            }

            match scopes.parent(scope) {
                Some(parent) => scope = parent,
                None => return None,
            }
        }
    }

    pub fn enter_scope(&mut self) {
        // no-op
    }

    pub fn exit_scope(&mut self, scope: ScopeId) {
        for i in (0..self.symbols.len()).rev() {
            let sym = &self.symbols[i];
            if sym.scope_id != scope {
                continue;
            }

            let key = (sym.namespace, sym.scope_id, sym.name.clone());

            if let Some(prev) = sym.shadowed {
                self.lookup_map.insert(key, prev);
            } else {
                self.lookup_map.remove(&key);
            }
        }
    }

    pub fn get_symbol(&self, id: SymbolId) -> Option<&SymbolEntry> {
        self.symbols.get(id)
    }
}