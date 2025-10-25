use crate::bytecode::chunk::{Chunk, FunctionChunk};

pub mod opcode;
pub mod value;
pub mod chunk;

pub type Index = usize;
pub type Arity = u8;

pub mod prelude {
    pub use super::opcode::*;
    pub use super::value::*;
    pub use super::chunk::*;

    pub use super::{Arity, Index, Bytecode};
}

#[derive(Debug, Clone)]
pub struct Bytecode {
    pub functions: Vec<FunctionChunk>,
    pub top_level: Chunk,
}