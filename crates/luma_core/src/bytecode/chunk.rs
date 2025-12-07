use std::fmt::Debug;

use crate::bytecode::{opcode::Instruction, value::BytecodeValue};

#[derive(Default, Clone)]
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

pub type Arity = u8;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub chunk: Chunk,
    pub arity: Arity,
    pub kind: FunctionKind,
    pub upvalues: Vec<UpvalueDescriptor>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpvalueDescriptor {
    pub is_local: bool,   // true = capture parent local
    pub index: usize,       // slot in locals or upvalues of parent
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("Chunk");

        dbg.field("instructions", &{
            struct Instrs<'a>(&'a [Instruction]);

            impl Debug for Instrs<'_> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let mut list = f.debug_list();

                    for (i, instr) in self.0.iter().enumerate() {
                        struct Line<'a> {
                            i: usize,
                            instr: &'a Instruction,
                        }

                        impl Debug for Line<'_> {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                write!(f, "{:04}. {:?}", self.i, self.instr)
                            }
                        }

                        list.entry(&Line { i, instr });
                    }

                    list.finish()
                }
            }
            Instrs(&self.instructions)
        });

        dbg.field("constants", &self.constants);
        dbg.field("local_count", &self.local_count);
        dbg.finish()
    }
}