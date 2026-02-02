mod opcode;
pub use opcode::Opcode;

pub struct Bytecode {
    pub instructions: Vec<Opcode>,
}

impl Bytecode {
    pub fn as_bytes(&self) -> Vec<u8> {
        // let mut bytes = Vec::new();
        todo!("bytecode serialization")
    }
}
