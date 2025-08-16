use std::{collections::HashMap, fmt::Debug};

use luma_core::ast::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub depth: u16,
    pub ty: Type,
    pub id: u32,
    pub shadow_index: Option<usize>, // index of the shadowed symbol if exists
}

#[derive(Default)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
    map: HashMap<String, usize>, // identifier to index map for fast lookup
    scope_stack: Vec<usize>, // used to track how much to pop from the symbol table on scope exit
    depth: u16,
}

impl Debug for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SymbolTable")
            .field("symbols", &self.symbols.len())
            .field("map", &self.map.len())
            .field("scope_stack", &self.scope_stack.len())
            .field("depth", &self.depth)
            .finish()
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
            map: HashMap::new(),
            scope_stack: Vec::new(),
            depth: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        self.depth += 1;
        self.scope_stack.push(self.symbols.len());
    }

    pub fn leave_scope(&mut self) {
        let Some(start) = self.scope_stack.pop() else {
            panic!("Cannot leave scope when no scope is active"); // TODO: Tracing
        };

        let indexes = (start..self.symbols.len()).rev();
        for i in indexes {
            let sym = &self.symbols[i];

            // handle identifier map
            if let Some(shadow_index) = sym.shadow_index {
                self.map.insert(sym.name.clone(), shadow_index);
            } else {
                self.map.remove(&sym.name);
            }
        }

        self.symbols.truncate(start);
        self.depth -= 1;
    }

    pub fn declare(&mut self, name: String, ty: Type) -> usize {
        let index = self.symbols.len();
        let prev = self.map.insert(name.clone(), index);

        self.symbols.push(Symbol {
            name,
            depth: self.depth,
            ty,
            id: index as u32,
            shadow_index: prev,
        });

        index
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.map.get(name).and_then(|&index| self.symbols.get(index))
    }
}