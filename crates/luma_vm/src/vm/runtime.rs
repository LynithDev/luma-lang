use crate::{frames::Frames, heap::Heap, stack::Stack};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuntimeOptions {
    pub max_frames: usize,
    pub max_stack_size: usize,
}

impl Default for RuntimeOptions {
    fn default() -> Self {
        Self {
            max_frames: 128,
            max_stack_size: 65536,
        }
    }
}

#[derive(Debug)]
pub struct RuntimeContext {
    pub frames: Frames,
    pub stack: Stack,
    pub heap: Heap,
}

impl RuntimeContext {
    #[allow(clippy::new_without_default)]
    pub fn new(options: RuntimeOptions) -> Self {
        Self {
            frames: Frames::new(options.max_frames),
            stack: Stack::new(options.max_stack_size),
            heap: Heap::new(),
        }
    }
}
