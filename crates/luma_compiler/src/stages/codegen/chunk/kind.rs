use luma_diagnostic::{CompilerResult, error};

use crate::{bytecode::Opcode, stages::codegen::CodegenError};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CodeChunk {
    instructions: Vec<Opcode>,
    pub max_locals: usize,
}

impl CodeChunk {
    pub fn emit(&mut self, opcode: Opcode) -> CompilerResult<u16> {
        let index = self.instr_len();
        if index == u16::MAX {
            return Err(error!(CodegenError::ChunkTooLarge));
        }

        self.instructions.push(opcode);
        Ok(index)
    }

    pub fn patch(&mut self, index: u16, opcode: Opcode) -> CompilerResult<()> {
        if index >= self.instr_len() {
            return Err(error!(CodegenError::InvalidPatchPosition { position: index }));
        }

        self.instructions[index as usize] = opcode;
        Ok(())
    }

    #[inline]
    pub const fn instr_len(&self) -> u16 {
        self.instructions.len() as u16
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    #[inline]
    pub fn last(&self) -> Option<&Opcode> {
        self.instructions.last()
    }

    #[inline]
    pub fn at(&self, index: u16) -> Option<&Opcode> {
        self.instructions.get(index as usize)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionChunk {
    pub code: CodeChunk,
    pub arity: usize,
}