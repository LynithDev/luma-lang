use crate::stages::codegen::chunk::TopLevelChunk;

mod opcode;
use luma_core::CodeSourceId;
pub use opcode::Opcode;

mod value;
pub use value::BytecodeValue;

#[derive(Debug)]
pub struct ModuleBytecode {
    pub source_id: CodeSourceId,
    pub chunk: TopLevelChunk,
}

impl ModuleBytecode {
    pub fn as_bytes(&self) -> Vec<u8> {
        // let mut bytes = Vec::new();
        todo!("bytecode serialization")
    }
}
