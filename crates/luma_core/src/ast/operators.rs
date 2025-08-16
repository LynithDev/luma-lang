#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Binary(BinaryOperator),
    Logical(LogicalOperator),
    Comparison(ComparisonOperator),
    Unary(UnaryOperator),
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Binary(op) => write!(f, "{op}"),
            Operator::Logical(op) => write!(f, "{op}"),
            Operator::Comparison(op) => write!(f, "{op}"),
            Operator::Unary(op) => write!(f, "{op}"),
        }
    }
}

#[derive(crate::Display, Debug, Clone, Copy, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum BinaryOperator {
    Assignment,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
}

#[derive(crate::Display, Debug, Clone, Copy, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(crate::Display, Debug, Clone, Copy, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum ComparisonOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}



#[derive(crate::Display, Debug, Clone, Copy, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum UnaryOperator {
    Not,
    Negative,
    Positive,
    BitwiseNot,
}
