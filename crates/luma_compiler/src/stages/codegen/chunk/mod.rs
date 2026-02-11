use crate::bytecode::{BytecodeValue, Opcode};

mod builder;
mod env;

pub use builder::*;
pub use env::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CodeChunk {
    pub instructions: Vec<Opcode>,
    pub max_locals: usize,
    pub constants: Vec<BytecodeValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelChunk {
    pub code: CodeChunk,
    pub functions: Vec<FunctionChunk>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionChunk {
    pub code: CodeChunk,
    pub arity: usize,
    pub nested_functions: Vec<FunctionChunk>,
}