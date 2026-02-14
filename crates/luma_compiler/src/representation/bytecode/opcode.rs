#[derive(strum::Display, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    // ##########################
    // ###  stack operations  ###
    // ##########################

    /// get local variable
    GetLocal(u16) = 0x01,

    /// set local variable
    SetLocal(u16) = 0x02,

    /// remove top of stack
    Pop = 0x03,

    /// duplicate top of stack
    Dup = 0x04,

    /// return from function
    Return = 0x05,

    // ###########################
    // ###  values / literals  ###
    // ###########################

    /// load constant from constant pool
    LoadConst(u16),

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

}