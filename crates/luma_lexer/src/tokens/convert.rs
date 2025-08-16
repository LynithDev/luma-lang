use luma_core::ast::{BinaryOperator, ComparisonOperator, LogicalOperator, Operator, UnaryOperator};
use crate::tokens::OperatorKind;

impl OperatorKind {
    pub fn is_assign_op(&self) -> bool {
        matches!(
            self,
            OperatorKind::Equals
            | OperatorKind::PlusEquals
            | OperatorKind::MinusEquals
            | OperatorKind::AsteriskEquals
            | OperatorKind::SlashEquals
            | OperatorKind::PercentEquals
            | OperatorKind::AndEquals
            | OperatorKind::OrEquals
            | OperatorKind::BitwiseAndEquals
            | OperatorKind::BitwiseOrEquals
            | OperatorKind::BitwiseXorEquals
            | OperatorKind::ShiftLeftEquals
            | OperatorKind::ShiftRightEquals
        )
    }

    pub fn as_assign_operator(&self) -> Option<Operator> {
        Some(match self {
            OperatorKind::Equals => Operator::Binary(BinaryOperator::Assignment),
            OperatorKind::PlusEquals => Operator::Binary(BinaryOperator::Add),
            OperatorKind::MinusEquals => Operator::Binary(BinaryOperator::Subtract),
            OperatorKind::AsteriskEquals => Operator::Binary(BinaryOperator::Multiply),
            OperatorKind::SlashEquals => Operator::Binary(BinaryOperator::Divide),
            OperatorKind::PercentEquals => Operator::Binary(BinaryOperator::Modulo),
            OperatorKind::AndEquals => Operator::Logical(LogicalOperator::And),
            OperatorKind::OrEquals => Operator::Logical(LogicalOperator::Or),
            OperatorKind::BitwiseAndEquals => Operator::Binary(BinaryOperator::BitwiseAnd),
            OperatorKind::BitwiseOrEquals => Operator::Binary(BinaryOperator::BitwiseOr),
            OperatorKind::BitwiseXorEquals => Operator::Binary(BinaryOperator::BitwiseXor),
            OperatorKind::ShiftLeftEquals => Operator::Binary(BinaryOperator::ShiftLeft),
            OperatorKind::ShiftRightEquals => Operator::Binary(BinaryOperator::ShiftRight),
            _ => return None,
        })
    }
}

impl From<crate::tokens::LiteralKind> for luma_core::ast::LiteralKind {
    fn from(kind: crate::tokens::LiteralKind) -> Self {
        match kind {
            crate::tokens::LiteralKind::Integer => Self::Integer,
            crate::tokens::LiteralKind::Decimal => Self::Decimal,
            crate::tokens::LiteralKind::String => Self::String,
            crate::tokens::LiteralKind::Boolean => Self::Boolean,
        }
    }
}

impl TryFrom<OperatorKind> for BinaryOperator {
    type Error = ();

    fn try_from(kind: OperatorKind) -> Result<Self, Self::Error> {
        Ok(match kind {
            OperatorKind::Plus => BinaryOperator::Add,
            OperatorKind::Minus => BinaryOperator::Subtract,
            OperatorKind::Asterisk => BinaryOperator::Multiply,
            OperatorKind::Slash => BinaryOperator::Divide,
            OperatorKind::Percent => BinaryOperator::Modulo,
            OperatorKind::BitwiseAnd => BinaryOperator::BitwiseAnd,
            OperatorKind::BitwiseOr => BinaryOperator::BitwiseOr,
            OperatorKind::BitwiseXor => BinaryOperator::BitwiseXor,
            OperatorKind::ShiftLeft => BinaryOperator::ShiftLeft,
            OperatorKind::ShiftRight => BinaryOperator::ShiftRight,

            OperatorKind::PlusEquals => BinaryOperator::Add,
            _ => return Err(()),
        })
    }
}

impl TryFrom<OperatorKind> for LogicalOperator {
    type Error = ();

    fn try_from(kind: OperatorKind) -> Result<Self, Self::Error> {
        Ok(match kind {
            OperatorKind::And => LogicalOperator::And,
            OperatorKind::Or => LogicalOperator::Or,
            _ => return Err(()),
        })
    }
}

impl TryFrom<OperatorKind> for ComparisonOperator {
    type Error = ();
    
    fn try_from(kind: OperatorKind) -> Result<Self, Self::Error> {
        Ok(match kind {
            OperatorKind::Equals => ComparisonOperator::Equals,
            OperatorKind::NotEquals => ComparisonOperator::NotEquals,
            OperatorKind::GreaterThan => ComparisonOperator::GreaterThan,
            OperatorKind::GreaterThanOrEqual => ComparisonOperator::GreaterThanOrEqual,
            OperatorKind::LessThan => ComparisonOperator::LessThan,
            OperatorKind::LessThanOrEqual => ComparisonOperator::LessThanOrEqual,
            _ => return Err(()),
        })
    }
}

impl TryFrom<OperatorKind> for UnaryOperator {
    type Error = ();

    fn try_from(kind: OperatorKind) -> Result<Self, Self::Error> {
        Ok(match kind {
            OperatorKind::Not => UnaryOperator::Not,
            OperatorKind::Minus => UnaryOperator::Negative,
            OperatorKind::Plus => UnaryOperator::Positive,
            OperatorKind::BitwiseNot => UnaryOperator::BitwiseNot,
            _ => return Err(()),
        })
    }
}
