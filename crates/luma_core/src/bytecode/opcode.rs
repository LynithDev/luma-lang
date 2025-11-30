use std::fmt::Debug;

use luma_macros::Display;

use crate::{bytecode::{ArityRef, IndexRef}, Cursor};

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OpCode {
    // binary operators
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,

    // comparison operators
    Equal,
    GreaterThan,
    LesserThan,
    GreaterThanEqual,
    LesserThanEqual,
    NotEqual,
    
    // logical operators
    And,
    Or,
    Negate,
    Not,
    BitNot,

    // literals
    Const(IndexRef),
    Closure(IndexRef, Option<IndexRef>),

    // flow control
    Return,
    Call(ArityRef),
    Jump(IndexRef),
    JumpIfFalse(IndexRef),
    
    // stack operations
    Dup,
    GetLocal(IndexRef),
    SetLocal(IndexRef),
    GetUpvalue(IndexRef),
    SetUpvalue(IndexRef),
    Pop,
    PopLocals(usize),
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
        // f.write_str(&format!("{}. ", self.cursor))?;
        f.write_str(&format!("{:?}", self.opcode))?;
        Ok(())
    }
}
