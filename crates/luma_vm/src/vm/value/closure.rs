use luma_core::bytecode::chunk::FunctionChunk;

use crate::{frames::Upvalue, slot_array::SlotArray};

#[derive(Debug, Clone)]
pub struct Closure {
    pub function: *const FunctionChunk,
    pub upvalues: SlotArray<Upvalue>,
}

impl PartialEq for Closure {
    fn eq(&self, other: &Self) -> bool {
        self.function == other.function
    }
}

impl Eq for Closure {}

impl std::hash::Hash for Closure {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.function.hash(state);
    }
}