use std::collections::HashMap;

use luma_diagnostic::{CompilerResult, error};

use crate::{bytecode::BytecodeValue, stages::codegen::CodegenError};

pub type ConstSlot = u16;

#[derive(Debug)]
pub struct ConstantTable {
    pub constants: Vec<BytecodeValue>,
    constants_lookup: HashMap<BytecodeValue, ConstSlot>,
}

impl ConstantTable {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            constants_lookup: HashMap::new(),
        }
    }

    /// Adds a constant (if it doesn't already exist) to the constant pool
    /// and returns its index
    pub fn add_constant(&mut self, value: BytecodeValue) -> CompilerResult<ConstSlot> {
        if let Some(&index) = self.constants_lookup.get(&value) {
            return Ok(index);
        }

        let index = self.constants.len();

        if index >= ConstSlot::MAX as usize {
            return Err(error!(CodegenError::TooManyConstants));
        }

        let index = index as ConstSlot;
        self.constants.push(value.clone());
        self.constants_lookup.insert(value, index);

        Ok(index)
    }

    /// Retrieves a constant by its index
    pub fn get_constant(&self, index: ConstSlot) -> Option<&BytecodeValue> {
        self.constants.get(index as usize)
    }
}