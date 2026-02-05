use crate::bytecode::{BytecodeValue, Opcode};

pub mod scope;
mod env;
mod builder;

pub use env::*;
pub use builder::*;

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