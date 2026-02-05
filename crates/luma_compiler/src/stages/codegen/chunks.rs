use crate::bytecode::Opcode;

#[derive(Debug, Clone, PartialEq)]
pub struct CodeChunk {
    pub instructions: Vec<Opcode>,
    pub max_locals: usize,
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