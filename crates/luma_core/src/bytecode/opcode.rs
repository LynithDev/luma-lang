use std::fmt::Debug;

use luma_macros::Display;

use crate::{bytecode::{ArityRef, IndexRef}, Cursor};

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OpCode {
    // ##########################
    // ###  binary operators  ###
    // ##########################
    
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

    // ##############################
    // ###  comparison operators  ###
    // ##############################

    Equal,
    GreaterThan,
    LesserThan,
    GreaterThanEqual,
    LesserThanEqual,
    NotEqual,
    
    // ###########################
    // ###  logical operators  ###
    // ###########################

    And,
    Or,
    Negate,
    Not,
    BitNot,

    // ###########################
    // ###  values / literals  ###
    // ###########################

    /// load constant from constant pool
    Const(IndexRef),

    /// create a closure from a function in the constant pool
    Closure(IndexRef, Option<IndexRef>),

    // ######################
    // ###  flow control  ###
    // ######################

    /// for returning a value from a function
    Return,
    
    /// for returning from a function with no return value
    ReturnUnit,
    
    /// for calling a function
    Call(ArityRef),

    /// goto instruction pointer
    Jump(IndexRef),

    /// jump if top of stack is false
    JumpIfFalse(IndexRef),
    
    // ##########################
    // ###  stack operations  ###
    // ##########################

    /// duplicate top of stack
    Dup,

    /// get local variable
    GetLocal(IndexRef),

    /// set local variable
    SetLocal(IndexRef),

    /// get upvalue
    GetUpvalue(IndexRef),

    /// set upvalue
    SetUpvalue(IndexRef),

    /// remove top of stack
    Pop,

    /// remove N slots from stack
    PopMul(usize),
}

impl OpCode {
    #[inline(always)]
    pub fn is_return(&self) -> bool {
        matches!(self,
            OpCode::Return |
            OpCode::ReturnUnit
        )
    }

    #[inline(always)]
    pub fn is_jump(&self) -> bool {
        matches!(self,
            OpCode::Jump(_) |
            OpCode::JumpIfFalse(_)
        )
    }
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
