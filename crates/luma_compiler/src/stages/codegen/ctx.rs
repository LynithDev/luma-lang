use std::collections::{HashMap, HashSet};

use luma_diagnostic::{CompilerResult, LumaError};

use crate::{bytecode::{BytecodeValue, Opcode}, stages::codegen::error::CodegenErrorKind};

#[derive(Default)]
pub struct BytecodeGenCtx {
    pub instructions: Vec<Opcode>,

    /// symbol id -> stack slot index
    pub locals: HashMap<usize, u16>,
    /// the max number of locals that could be allocated in this context
    pub max_locals: usize,

    pub constants: HashSet<BytecodeValue>,

    /*
     * probably implement some sort of scope tracking, this way I can optimise the max_locals pre-allocation
     */
}

impl BytecodeGenCtx {
    pub fn emit(&mut self, opcode: Opcode) {
        self.instructions.push(opcode);
    }

    pub fn patch(&mut self, position: usize, opcode: Opcode) -> CompilerResult<()> {
        if position >= self.len() {
            return Err(LumaError::new(
                CodegenErrorKind::InvalidPatchPosition(position)
            ));
        }

        self.instructions[position] = opcode;
        Ok(())
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.instructions.len()
    }

    #[must_use]
    pub fn local_count(&self) -> u16 {
        self.locals.len() as u16
    }

    pub fn declare_local(&mut self, symbol_id: usize) -> u16 {
        let slot = self.local_count();
        self.locals.insert(symbol_id, slot);

        // todo: optimize max_locals tracking with scopes
        if (slot as usize) + 1 > self.max_locals {
            self.max_locals = (slot as usize) + 1;
        }

        slot
    }

    pub fn declare_constant(&mut self, value: BytecodeValue) -> u16 {
        let id = self.constants.len() as u16;
        self.constants.insert(value);
        id
    }
}
