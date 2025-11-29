use luma_core::bytecode::chunk::Chunk;

use crate::{value::{Closure, StackValue}, VmError, VmResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameSource {
    TopLevel(*mut Chunk),
    Closure(*mut Closure),
}

#[derive(Debug)]
pub struct CallFrame {
    pub source: FrameSource,
    pub instr_pointer: usize,
    pub base: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Upvalue {
    Open(*mut StackValue),    // points into some stack frame slot
    Closed(StackValue),       // heap-allocated copy after the slot’s frame ends
}

impl CallFrame {
    pub fn get_chunk(&self) -> &Chunk {
        unsafe {
            match self.source {
                FrameSource::TopLevel(chunk) => &*chunk,
                FrameSource::Closure(closure) => {
                    let closure = &*closure;
                    let func_chunk = &*closure.function;
                    &func_chunk.chunk
                }
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

    pub fn last(&self) -> VmResult<&CallFrame> {
        self.inner.last().ok_or(VmError::NoActiveCallFrame)
    }

    pub fn last_mut(&mut self) -> VmResult<&mut CallFrame> {
        self.inner.last_mut().ok_or(VmError::NoActiveCallFrame)
    }

    pub fn get(&self, index: usize) -> Option<&CallFrame> {
        self.inner.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut CallFrame> {
        self.inner.get_mut(index)
    }

    pub fn try_get(&self, index: usize) -> VmResult<&CallFrame> {
        self.get(index).ok_or(VmError::NoCallFrameAtIndex(index))
    }

    pub fn try_get_mut(&mut self, index: usize) -> VmResult<&mut CallFrame> {
        self.get_mut(index).ok_or(VmError::NoCallFrameAtIndex(index))
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
