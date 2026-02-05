use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum LoweringErrorKind {
    #[error("expected scope id to be present, but it was not found'")]
    MissingScopeId,
    #[error("expected symbol id to be present, but it was not found'")]
    MissingSymbolId,
    #[error("expected type to be known")]
    UnknownType,
    #[error("invalid node for int lowering: '{0}'")]
    InvalidLiteralConversion(String),
    #[error("integer literal '{amount}' is too large to fit in '{target}'")]
    IntegerOverflow {
        amount: u64,
        target: String,
    },
    #[error("float literal '{amount}' is too large to fit in '{target}'")]
    FloatOverflow {
        amount: f64,
        target: String,
    },
    #[error("cannot cast from '{from}' to '{to}'")]
    InvalidCast {
        from: String,
        to: String,
    },
}