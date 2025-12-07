use crate::{bytecode::chunk::{Chunk, FunctionChunk}};

pub mod opcode;
pub mod value;
pub mod chunk;

pub mod prelude {
    pub use super::opcode::*;
    pub use super::value::*;
    pub use super::chunk::*;

    pub use super::{Bytecode};
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Bytecode {
    pub functions: Vec<FunctionChunk>,
    pub top_level: Chunk,
}

impl Bytecode {
    pub fn new() -> Self {
        Self::default()
    }
}