use luma_core::bytecode::{chunk::Chunk, IndexRef};

use crate::{locals::Locals, ProgramSource, VmError, VmResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChunkRef {
    TopLevel,
    Function(IndexRef),
}

#[derive(Debug)]
pub struct CallFrame {
    pub source_index: IndexRef,
    pub chunk_ref: ChunkRef,
    pub instr_pointer: usize,
    pub base: usize,
    pub locals: Locals,
}

impl CallFrame {
    pub fn try_get_chunk<'a>(&'a self, sources: &'a [ProgramSource]) -> VmResult<&'a Chunk> {
        let source = sources
            .get(*self.source_index)
            .ok_or(VmError::IndexOutOfBounds(*self.source_index))?;

        match self.chunk_ref {
            ChunkRef::TopLevel => Ok(&source.bytecode.top_level),
            ChunkRef::Function(func_index) => {
                let function_chunk = source
                    .bytecode
                    .functions
                    .get(*func_index)
                    .ok_or(VmError::IndexOutOfBounds(*func_index))?;
                Ok(&function_chunk.chunk)
            }
        }
    }
}

#[derive(Debug)]
pub struct Frames {
    inner: Vec<CallFrame>,
}

impl Frames {
    pub fn new(len: usize) -> Self {
        Self {
            inner: Vec::with_capacity(len),
        }
    }

    pub fn push(&mut self, frame: CallFrame) -> VmResult<()> {
        let index = self.inner.len();
        if index >= self.inner.capacity() {
            return Err(VmError::MaxFrameCountExceeded);
        }
        
        self.inner.push(frame);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<CallFrame> {
        self.inner.pop()
    }

    pub fn last_mut(&mut self) -> VmResult<&mut CallFrame> {
        self.inner.last_mut().ok_or(VmError::NoActiveCallFrame)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }
}
