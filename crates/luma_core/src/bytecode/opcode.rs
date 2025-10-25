use std::fmt::Debug;

use luma_macros::Display;

use crate::{bytecode::{Arity, Index}, Cursor};

#[derive(Display, Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpCode {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    Greater,
    Less,

    Negate,
    Not,

    True,
    False,
    Const(Index),

    Return,
    Call(Arity),
    Jump(Index),
    JumpIfFalse(Index),
    Pop,

    GetLocal(Index),
    SetLocal(Index),
}

#[derive(Clone, PartialEq)]
pub struct Instruction {
    pub opcode: OpCode,
    pub cursor: Cursor,
}

impl Instruction {
    pub fn new(opcode: OpCode, cursor: Cursor) -> Self {
        Self { 
            opcode,
            cursor
        }
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}. ", self.cursor))?;
        f.write_str(&format!("{:?}", self.opcode))?;
        Ok(())
    }
}
