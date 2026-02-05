#[derive(strum::Display, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
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
    LoadConst(u16),
    
    // ##########################
    // ###  stack operations  ###
    // ##########################

    /// duplicate top of stack
    Dup,

    /// get local variable
    GetLocal(u16),

    /// set local variable
    SetLocal(u16),

    /// remove top of stack
    Pop,
}