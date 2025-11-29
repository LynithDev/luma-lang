use crate::{arena::Arena, frames::Frames, heap::Heap, stack::Stack, value::Closure};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuntimeOptions {
    pub max_frames: usize,
    pub max_stack_size: usize,
}

impl Default for RuntimeOptions {
    fn default() -> Self {
        Self {
            max_frames: u8::MAX as usize,
            max_stack_size: u16::MAX as usize,
        }
    }
}

#[derive(Debug)]
pub struct RuntimeContext {
    pub frames: Frames,
    pub stack: Stack,
    pub heap: Heap,
    pub closures: Arena<Closure>,
}

impl RuntimeContext {
    #[allow(clippy::new_without_default)]
    pub fn new(options: RuntimeOptions) -> Self {
        Self {
            frames: Frames::new(options.max_frames),
            stack: Stack::new(options.max_stack_size),
            heap: Heap::new(),
            closures: Arena::new(),
        }
    }
}
