use crate::bytecode::Opcode;

mod builder;
mod env;

pub use builder::*;
pub use env::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CodeChunk {
    pub instructions: Vec<Opcode>,
    pub max_locals: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionChunk {
    pub code: CodeChunk,
    pub arity: usize,
}