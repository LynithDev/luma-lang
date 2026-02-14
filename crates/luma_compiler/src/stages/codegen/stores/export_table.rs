use std::collections::HashMap;

use crate::SymbolId;

#[derive(Debug)]
pub struct ExportTable {
    pub functions: HashMap<SymbolId, u16>,
    pub variables: HashMap<SymbolId, u16>,
}

impl ExportTable {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, symbol: SymbolId, index: u16) {
        self.functions.insert(symbol, index);
    }

    pub fn get_function(&self, symbol: &SymbolId) -> Option<u16> {
        self.functions.get(symbol).copied()
    }

    pub fn get_functions(&self) -> &HashMap<SymbolId, u16> {
        &self.functions
    }

    pub fn add_variable(&mut self, symbol: SymbolId, index: u16) {
        self.variables.insert(symbol, index);
    }

    pub fn get_variable(&self, symbol: &SymbolId) -> Option<u16> {
        self.variables.get(symbol).copied()
    }
    
    pub fn get_variables(&self) -> &HashMap<SymbolId, u16> {
        &self.variables
    }
}