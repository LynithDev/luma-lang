use crate::bytecode::{opcode::Instruction, value::BytecodeValue, ArityRef, IndexRef};

#[derive(Default, Debug, Clone)]
pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<BytecodeValue>,
    pub local_count: usize,
}

impl PartialEq for Chunk {
    fn eq(&self, other: &Self) -> bool {
        self.instructions == other.instructions && self.constants == other.constants
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionKind {
    Function,
    Method,
    Initializer,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub chunk: Chunk,
    pub arity: ArityRef,
    pub kind: FunctionKind,
    pub upvalues: Vec<UpvalueDescriptor>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpvalueDescriptor {
    pub is_local: bool,   // true = capture parent local
    pub index: IndexRef,       // slot in locals or upvalues of parent
}
