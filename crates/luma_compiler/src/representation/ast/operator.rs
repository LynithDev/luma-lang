use std::{fmt::Display, str::FromStr};

use luma_core::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operator {
    pub kind: OperatorKind,
    pub span: Span,
}

impl Operator {
    #[must_use]
    pub const fn new(span: Span, kind: OperatorKind) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorKind {
    // other
    Assign,
    Not,

    // arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    
    // logic
    And,
    Or,

    AndAssign,
    OrAssign,

    // comparison
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,

    // bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,

    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    ShiftLeftAssign,
    ShiftRightAssign,
}

impl OperatorKind {
    #[must_use]
    pub const fn is_prefix(&self) -> bool {
        matches!(self, OperatorKind::Not | OperatorKind::Subtract)
    }

    #[must_use]
    pub const fn is_infix(&self) -> bool {
        !self.is_prefix()
    }

    #[must_use]
    pub const fn is_arithmetic(&self) -> bool {
        matches!(
            self,
            OperatorKind::Add
                | OperatorKind::Subtract
                | OperatorKind::Multiply
                | OperatorKind::Divide
                | OperatorKind::Modulo
        )
    }

    #[must_use]
    pub const fn is_logic(&self) -> bool {
        matches!(self, OperatorKind::And | OperatorKind::Or)
    }

    #[must_use]
    pub const fn is_comparison(&self) -> bool {
        matches!(
            self,
            OperatorKind::Equal
                | OperatorKind::NotEqual
                | OperatorKind::LessThan
                | OperatorKind::GreaterThan
                | OperatorKind::LessThanOrEqual
                | OperatorKind::GreaterThanOrEqual
        )
    }

    #[must_use]
    pub const fn is_bitwise(&self) -> bool {
        matches!(
            self,
            OperatorKind::BitwiseAnd
                | OperatorKind::BitwiseOr
                | OperatorKind::BitwiseXor
                | OperatorKind::ShiftLeft
                | OperatorKind::ShiftRight
        )
    }

    #[must_use]
    pub const fn is_assignment(&self) -> bool {
        matches!(
            self,
            OperatorKind::Assign
                | OperatorKind::AddAssign
                | OperatorKind::SubtractAssign
                | OperatorKind::MultiplyAssign
                | OperatorKind::DivideAssign
                | OperatorKind::ModuloAssign
                
                | OperatorKind::AndAssign
                | OperatorKind::OrAssign

                | OperatorKind::BitwiseAndAssign
                | OperatorKind::BitwiseOrAssign
                | OperatorKind::BitwiseXorAssign
                | OperatorKind::ShiftLeftAssign
                | OperatorKind::ShiftRightAssign
        )
    }
}

impl FromStr for OperatorKind {
    type Err = ();
    
    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            // others
            "=" => Ok(OperatorKind::Assign),
            "!" => Ok(OperatorKind::Not),

            // arithmetic
            "+" => Ok(OperatorKind::Add),
            "-" => Ok(OperatorKind::Subtract),
            "*" => Ok(OperatorKind::Multiply),
            "/" => Ok(OperatorKind::Divide),
            "%" => Ok(OperatorKind::Modulo),

            "+= " => Ok(OperatorKind::AddAssign),
            "-= " => Ok(OperatorKind::SubtractAssign),
            "*= " => Ok(OperatorKind::MultiplyAssign),
            "/= " => Ok(OperatorKind::DivideAssign),
            "%= " => Ok(OperatorKind::ModuloAssign),

            // logic
            "&&" => Ok(OperatorKind::And),
            "||" => Ok(OperatorKind::Or),

            "&&= " => Ok(OperatorKind::AndAssign),
            "||= " => Ok(OperatorKind::OrAssign),

            // comparison
            "==" => Ok(OperatorKind::Equal),
            "!=" => Ok(OperatorKind::NotEqual),
            "<" => Ok(OperatorKind::LessThan),
            ">" => Ok(OperatorKind::GreaterThan),
            "<=" => Ok(OperatorKind::LessThanOrEqual),
            ">=" => Ok(OperatorKind::GreaterThanOrEqual),
            
            // bitwise
            "&" => Ok(OperatorKind::BitwiseAnd),
            "|" => Ok(OperatorKind::BitwiseOr),
            "^" => Ok(OperatorKind::BitwiseXor),
            "<<" => Ok(OperatorKind::ShiftLeft),
            ">>" => Ok(OperatorKind::ShiftRight),
            
            "&= " => Ok(OperatorKind::BitwiseAndAssign),
            "|= " => Ok(OperatorKind::BitwiseOrAssign),
            "^= " => Ok(OperatorKind::BitwiseXorAssign),
            "<<= " => Ok(OperatorKind::ShiftLeftAssign),
            ">>= " => Ok(OperatorKind::ShiftRightAssign),

            _ => Err(()),
        }
    }
}

impl Display for OperatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // others
            OperatorKind::Assign => write!(f, "="),
            OperatorKind::Not => write!(f, "!"),

            // arithmetic
            OperatorKind::Add => write!(f, "+"),
            OperatorKind::Subtract => write!(f, "-"),
            OperatorKind::Multiply => write!(f, "*"),
            OperatorKind::Divide => write!(f, "/"),
            OperatorKind::Modulo => write!(f, "%"),

            OperatorKind::AddAssign => write!(f, "+="),
            OperatorKind::SubtractAssign => write!(f, "-="),
            OperatorKind::MultiplyAssign => write!(f, "*="),
            OperatorKind::DivideAssign => write!(f, "/="),
            OperatorKind::ModuloAssign => write!(f, "%="),

            // logic
            OperatorKind::And => write!(f, "&&"),
            OperatorKind::Or => write!(f, "||"),

            OperatorKind::AndAssign => write!(f, "&&="),
            OperatorKind::OrAssign => write!(f, "||="),
            
            // comparison
            OperatorKind::Equal => write!(f, "=="),
            OperatorKind::NotEqual => write!(f, "!="),
            OperatorKind::LessThan => write!(f, "<"),
            OperatorKind::GreaterThan => write!(f, ">"),
            OperatorKind::LessThanOrEqual => write!(f, "<="),
            OperatorKind::GreaterThanOrEqual => write!(f, ">="),
            
            // bitwise
            OperatorKind::BitwiseAnd => write!(f, "&"),
            OperatorKind::BitwiseOr => write!(f, "|"),
            OperatorKind::BitwiseXor => write!(f, "^"),
            OperatorKind::ShiftLeft => write!(f, "<<"),
            OperatorKind::ShiftRight => write!(f, ">>"),

            OperatorKind::BitwiseAndAssign => write!(f, "&="),
            OperatorKind::BitwiseOrAssign => write!(f, "|="),
            OperatorKind::BitwiseXorAssign => write!(f, "^="),
            OperatorKind::ShiftLeftAssign => write!(f, "<<="),
            OperatorKind::ShiftRightAssign => write!(f, ">>="),
        }
    }
}