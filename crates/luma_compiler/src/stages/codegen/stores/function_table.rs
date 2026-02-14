use std::collections::HashMap;

use crate::{SymbolId, stages::codegen::chunk::FunctionChunk};

#[derive(Debug)]
pub struct FunctionTable {
    pub functions: Vec<FunctionChunk>,
    lookup: HashMap<SymbolId, usize>,
}

impl FunctionTable {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            lookup: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, symbol_id: SymbolId, chunk: FunctionChunk) -> usize {
        let index = self.functions.len();
        self.functions.push(chunk);
        self.lookup.insert(symbol_id, index);
        index
    }

    pub fn get_function(&self, symbol_id: &SymbolId) -> Option<&FunctionChunk> {
        self.lookup.get(symbol_id).and_then(|&index| self.functions.get(index))
    }
}