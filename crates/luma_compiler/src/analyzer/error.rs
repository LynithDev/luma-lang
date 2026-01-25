use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum AnalyzerErrorKind {
    #[error("unresolved identifier: '{0}'")]
    UnresolvedIdentifier(String),
    #[error("unidentified symbol: '{0}'")]
    UnidentifiedSymbol(String),
    #[error("unresolved named type: '{0}'")]
    UnresolvedType(String),
    #[error("unresolved struct field '{field_name}' for struct '{struct_name}'")]
    UnresolvedStructField {
        struct_name: String,
        field_name: String,
    },
}