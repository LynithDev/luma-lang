use luma_core::bytecode::chunk::Chunk;

use crate::value::{Closure, StackValue};

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