use crate::stages::codegen::chunk::FunctionChunk;

mod opcode;
use luma_core::CodeSourceId;
pub use opcode::Opcode;

mod value;
pub use value::BytecodeValue;

#[derive(Debug)]
pub struct ModuleBytecode {
    pub source_id: CodeSourceId,
    pub constants: Vec<BytecodeValue>,
    pub functions: Vec<FunctionChunk>,
}

impl ModuleBytecode {
    pub fn get_init_chunk(&self) -> Option<&FunctionChunk> {
        self.functions.first()
    }
}
