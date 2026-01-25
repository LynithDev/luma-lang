use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    /// list of all declared symbols
    symbols: Vec<SymbolEntry>,
    /// stores mapping from (namespace, name) to symbol index
    lookup_map: HashMap<(SymbolNamespace, String), usize>,
    /// stack of scopes, each scope is a list of symbol ids
    scope_stack: Vec<usize>,
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
    name: String,
    /// index to previous shadowed symbol in the same namespace
    shadowed: Option<usize>,
    namespace: SymbolNamespace,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: Vec::new(),
            lookup_map: HashMap::new(),
            scope_stack: Vec::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.scope_stack.push(self.symbols.len());
    }

    pub fn exit_scope(&mut self) {
        let Some(start) = self.scope_stack.pop() else {
            panic!("attempted to exit scope when no scope is active.");
        };

        for sym in self.symbols[start..].iter().rev() {
            let key = (sym.namespace, sym.name.clone());

            if let Some(prev) = sym.shadowed {
                self.lookup_map.insert(key, prev);
            } else {
                self.lookup_map.remove(&key);
            }
        }
    }

    pub fn scoped(&mut self, f: impl FnOnce(&mut Self)) {
        self.enter_scope();
        f(self);
        self.exit_scope();
    }

    pub fn declare(&mut self, namespace: SymbolNamespace, name: String) -> usize {
        let shadowed = self.lookup_map.get(&(namespace, name.clone())).cloned();

        let id = self.symbols.len();

        self.symbols.push(SymbolEntry {
            name: name.clone(),
            shadowed,
            namespace,
        });

        self.lookup_map.insert((namespace, name), id);

        id
    }

    pub fn recache(&mut self, index: usize) {
        let Some(symbol) = self.get_symbol_by_index(index) else {
            panic!("attempted to redeclare non-existent symbol id {}", index);
        };
        
        let key = (symbol.namespace, symbol.name.clone());
        
        self.lookup_map.insert(key, index);

        if let Some(scope_stack) = self.scope_stack.last_mut() {
            *scope_stack -= 1;
        }
    }

    /// Looks up a symbol by its namespace and name, returning its symbol ID 
    pub fn lookup(&self, namespace: SymbolNamespace, name: &str) -> Option<usize> {
        self.lookup_map.get(&(namespace, name.to_string())).cloned()
    }

    pub fn get_symbol_by_index(&self, index: usize) -> Option<&SymbolEntry> {
        self.symbols.get(index)
    }
}
