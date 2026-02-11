use std::collections::HashMap;

use luma_diagnostic::{CompilerResult, error};

use crate::{
    bytecode::*,
    stages::codegen::{CodegenError, chunk::CodeChunk},
};

pub type LocalSlot = u16;
pub type ConstSlot = u16;

#[derive(Debug)]
pub struct ChunkBuilderEnv {
    pub chunk: CodeChunk,

    /// maps local variables to their slot index
    /// symbol_id -> slot_index
    local_slots: HashMap<usize, LocalSlot>,

    /// constants used in the chunk
    constants_lookup: HashMap<BytecodeValue, ConstSlot>,
}

impl ChunkBuilderEnv {
    pub fn new() -> Self {
        Self {
            chunk: CodeChunk::default(),
            local_slots: HashMap::new(),
            constants_lookup: HashMap::new(),
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

    /// Adds a constant (if it doesn't already exist) to the constant pool
    /// and returns its index
    pub fn add_constant(&mut self, value: BytecodeValue) -> CompilerResult<ConstSlot> {
        if let Some(&index) = self.constants_lookup.get(&value) {
            return Ok(index);
        }

        let index = self.chunk.constants.len();

        if index >= ConstSlot::MAX as usize {
            return Err(error!(CodegenError::TooManyConstants));
        }

        let index = index as ConstSlot;
        self.chunk.constants.push(value.clone());
        self.constants_lookup.insert(value, index);

        Ok(index)
    }

    /// Retrieves a constant by its index
    pub fn get_constant(&self, index: ConstSlot) -> Option<&BytecodeValue> {
        self.chunk.constants.get(index as usize)
    }

    /// Emits an opcode
    pub fn emit(&mut self, opcode: Opcode) {
        self.chunk.instructions.push(opcode);
    }

    /// Updates an opcode at a specific index
    pub fn patch_instr(&mut self, index: usize, opcode: Opcode) -> CompilerResult<()> {
        if index >= self.chunk.instructions.len() {
            return Err(error!(CodegenError::InvalidPatchPosition {
                position: index,
            }));
        }

        self.chunk.instructions[index] = opcode;
        Ok(())
    }
}
