use luma_core::CodeSourceId;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CompilerErrorKind {
    #[error("invalid source id: '{id:?}'")]
    InvalidSourceId {
        id: CodeSourceId,
    },
}