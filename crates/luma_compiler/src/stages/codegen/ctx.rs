use crate::bytecode::Opcode;

#[derive(Default)]
pub struct BytecodeGenCtx {
    pub instructions: Vec<Opcode>,
}