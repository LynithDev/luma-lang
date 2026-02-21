use std::fmt::Display;

use luma_core::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotOperator {
    pub kind: AnnotOperatorKind,
    pub span: Span,
}

impl AnnotOperator {
    #[must_use]
    pub const fn new(span: Span, kind: AnnotOperatorKind) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnnotOperatorKind {
    // other
    Not,

    // arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // logic
    And,
    Or,

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
}

impl AnnotOperatorKind {
    #[must_use]
    pub const fn is_prefix(&self) -> bool {
        matches!(self, AnnotOperatorKind::Not | AnnotOperatorKind::Subtract)
    }

    #[must_use]
    pub const fn is_infix(&self) -> bool {
        !self.is_prefix()
    }

    #[must_use]
    pub const fn is_arithmetic(&self) -> bool {
        matches!(
            self,
            AnnotOperatorKind::Add
                | AnnotOperatorKind::Subtract
                | AnnotOperatorKind::Multiply
                | AnnotOperatorKind::Divide
                | AnnotOperatorKind::Modulo
        )
    }

    #[must_use]
    pub const fn is_logic(&self) -> bool {
        matches!(self, AnnotOperatorKind::And | AnnotOperatorKind::Or)
    }

    #[must_use]
    pub const fn is_comparison(&self) -> bool {
        matches!(
            self,
            AnnotOperatorKind::Equal
                | AnnotOperatorKind::NotEqual
                | AnnotOperatorKind::LessThan
                | AnnotOperatorKind::GreaterThan
                | AnnotOperatorKind::LessThanOrEqual
                | AnnotOperatorKind::GreaterThanOrEqual
        )
    }

    #[must_use]
    pub const fn is_bitwise(&self) -> bool {
        matches!(
            self,
            AnnotOperatorKind::BitwiseAnd
                | AnnotOperatorKind::BitwiseOr
                | AnnotOperatorKind::BitwiseXor
                | AnnotOperatorKind::ShiftLeft
                | AnnotOperatorKind::ShiftRight
        )
    }
}

impl Display for AnnotOperatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // others
            AnnotOperatorKind::Not => write!(f, "!"),

            // arithmetic
            AnnotOperatorKind::Add => write!(f, "+"),
            AnnotOperatorKind::Subtract => write!(f, "-"),
            AnnotOperatorKind::Multiply => write!(f, "*"),
            AnnotOperatorKind::Divide => write!(f, "/"),
            AnnotOperatorKind::Modulo => write!(f, "%"),

            // logic
            AnnotOperatorKind::And => write!(f, "&&"),
            AnnotOperatorKind::Or => write!(f, "||"),

            // comparison
            AnnotOperatorKind::Equal => write!(f, "=="),
            AnnotOperatorKind::NotEqual => write!(f, "!="),
            AnnotOperatorKind::LessThan => write!(f, "<"),
            AnnotOperatorKind::GreaterThan => write!(f, ">"),
            AnnotOperatorKind::LessThanOrEqual => write!(f, "<="),
            AnnotOperatorKind::GreaterThanOrEqual => write!(f, ">="),

            // bitwise
            AnnotOperatorKind::BitwiseAnd => write!(f, "&"),
            AnnotOperatorKind::BitwiseOr => write!(f, "|"),
            AnnotOperatorKind::BitwiseXor => write!(f, "^"),
            AnnotOperatorKind::ShiftLeft => write!(f, "<<"),
            AnnotOperatorKind::ShiftRight => write!(f, ">>"),
        }
    }
}
