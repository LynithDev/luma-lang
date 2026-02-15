use std::collections::HashMap;

use luma_diagnostic::{CompilerResult, error};

use crate::stages::codegen::{CodegenError, chunk::CodeChunk};

pub type LocalSlot = u16;

#[derive(Debug)]
pub struct ChunkBuilderEnv {
    pub chunk: CodeChunk,

    /// maps local variables to their slot index
    /// symbol_id -> slot_index
    local_slots: HashMap<usize, LocalSlot>,
}

impl ChunkBuilderEnv {
    pub fn new() -> Self {
        Self {
            chunk: CodeChunk::default(),
            local_slots: HashMap::new(),
        }
    }

    /// Declares a new local variable and returns its slot index
    pub fn declare_local(&mut self, symbol_id: usize) -> CompilerResult<LocalSlot> {
        let slot_index = self.local_slots.len();

        if slot_index >= LocalSlot::MAX as usize {
            return Err(error!(CodegenError::TooManyLocals));
        }

        let slot_index = slot_index as LocalSlot;
        self.local_slots.insert(symbol_id, slot_index);

        // todo: proper max_locals counting with scope management
        self.chunk.max_locals += 1;

        Ok(slot_index)
    }

    /// Returns the slot index of the local variable if it exists
    pub fn resolve_local_slot(&self, symbol_id: &usize) -> CompilerResult<LocalSlot> {
        self.local_slots.get(symbol_id).copied().ok_or_else(|| {
            error!(CodegenError::UndefinedLocal {
                symbol_id: *symbol_id,
            })
        })
    }
}
