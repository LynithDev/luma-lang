use std::{collections::HashMap, hash::{DefaultHasher, Hash, Hasher}};

use crate::bytecode::{opcode::Instruction, value::BytecodeValue, Arity};

#[derive(Default, Debug, Clone)]
pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<BytecodeValue>,
    constants_lookup: HashMap<BytecodeValue, usize>,
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

    pub fn add_const(&mut self, value: BytecodeValue) -> usize {
        if let Some(&index) = self.constants_lookup.get(&value) {
            return index;
        }

        self.constants.push(value.clone());
        let index = self.constants.len() - 1;
        self.constants_lookup.insert(value, index);
        index
    }

    pub fn emit_instr(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
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
    pub arity: Arity,
    pub kind: FunctionKind,
}