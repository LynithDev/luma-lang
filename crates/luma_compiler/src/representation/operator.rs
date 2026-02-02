use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
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

impl Operator {
    #[must_use]
    pub const fn is_prefix(&self) -> bool {
        matches!(self, Operator::Not | Operator::Subtract)
    }

    #[must_use]
    pub const fn is_infix(&self) -> bool {
        !self.is_prefix()
    }

    #[must_use]
    pub const fn is_arithmetic(&self) -> bool {
        matches!(
            self,
            Operator::Add
                | Operator::Subtract
                | Operator::Multiply
                | Operator::Divide
                | Operator::Modulo
        )
    }

    #[must_use]
    pub const fn is_logic(&self) -> bool {
        matches!(self, Operator::And | Operator::Or)
    }

    #[must_use]
    pub const fn is_comparison(&self) -> bool {
        matches!(
            self,
            Operator::Equal
                | Operator::NotEqual
                | Operator::LessThan
                | Operator::GreaterThan
                | Operator::LessThanOrEqual
                | Operator::GreaterThanOrEqual
        )
    }

    #[must_use]
    pub const fn is_bitwise(&self) -> bool {
        matches!(
            self,
            Operator::BitwiseAnd
                | Operator::BitwiseOr
                | Operator::BitwiseXor
                | Operator::ShiftLeft
                | Operator::ShiftRight
        )
    }

    #[must_use]
    pub const fn is_assignment(&self) -> bool {
        matches!(
            self,
            Operator::Assign
                | Operator::AddAssign
                | Operator::SubtractAssign
                | Operator::MultiplyAssign
                | Operator::DivideAssign
                | Operator::ModuloAssign
                
                | Operator::AndAssign
                | Operator::OrAssign

                | Operator::BitwiseAndAssign
                | Operator::BitwiseOrAssign
                | Operator::BitwiseXorAssign
                | Operator::ShiftLeftAssign
                | Operator::ShiftRightAssign
        )
    }
}

impl FromStr for Operator {
    type Err = ();
    
    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            // others
            "=" => Ok(Operator::Assign),
            "!" => Ok(Operator::Not),

            // arithmetic
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            "%" => Ok(Operator::Modulo),

            "+= " => Ok(Operator::AddAssign),
            "-= " => Ok(Operator::SubtractAssign),
            "*= " => Ok(Operator::MultiplyAssign),
            "/= " => Ok(Operator::DivideAssign),
            "%= " => Ok(Operator::ModuloAssign),

            // logic
            "&&" => Ok(Operator::And),
            "||" => Ok(Operator::Or),

            "&&= " => Ok(Operator::AndAssign),
            "||= " => Ok(Operator::OrAssign),

            // comparison
            "==" => Ok(Operator::Equal),
            "!=" => Ok(Operator::NotEqual),
            "<" => Ok(Operator::LessThan),
            ">" => Ok(Operator::GreaterThan),
            "<=" => Ok(Operator::LessThanOrEqual),
            ">=" => Ok(Operator::GreaterThanOrEqual),
            
            // bitwise
            "&" => Ok(Operator::BitwiseAnd),
            "|" => Ok(Operator::BitwiseOr),
            "^" => Ok(Operator::BitwiseXor),
            "<<" => Ok(Operator::ShiftLeft),
            ">>" => Ok(Operator::ShiftRight),
            
            "&= " => Ok(Operator::BitwiseAndAssign),
            "|= " => Ok(Operator::BitwiseOrAssign),
            "^= " => Ok(Operator::BitwiseXorAssign),
            "<<= " => Ok(Operator::ShiftLeftAssign),
            ">>= " => Ok(Operator::ShiftRightAssign),

            _ => Err(()),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // others
            Operator::Assign => write!(f, "="),
            Operator::Not => write!(f, "!"),

            // arithmetic
            Operator::Add => write!(f, "+"),
            Operator::Subtract => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Modulo => write!(f, "%"),

            Operator::AddAssign => write!(f, "+="),
            Operator::SubtractAssign => write!(f, "-="),
            Operator::MultiplyAssign => write!(f, "*="),
            Operator::DivideAssign => write!(f, "/="),
            Operator::ModuloAssign => write!(f, "%="),

            // logic
            Operator::And => write!(f, "&&"),
            Operator::Or => write!(f, "||"),

            Operator::AndAssign => write!(f, "&&="),
            Operator::OrAssign => write!(f, "||="),
            
            // comparison
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::LessThan => write!(f, "<"),
            Operator::GreaterThan => write!(f, ">"),
            Operator::LessThanOrEqual => write!(f, "<="),
            Operator::GreaterThanOrEqual => write!(f, ">="),
            
            // bitwise
            Operator::BitwiseAnd => write!(f, "&"),
            Operator::BitwiseOr => write!(f, "|"),
            Operator::BitwiseXor => write!(f, "^"),
            Operator::ShiftLeft => write!(f, "<<"),
            Operator::ShiftRight => write!(f, ">>"),

            Operator::BitwiseAndAssign => write!(f, "&="),
            Operator::BitwiseOrAssign => write!(f, "|="),
            Operator::BitwiseXorAssign => write!(f, "^="),
            Operator::ShiftLeftAssign => write!(f, "<<="),
            Operator::ShiftRightAssign => write!(f, ">>="),
        }
    }
}