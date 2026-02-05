use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CodegenErrorKind {
    #[error("attempted to patch an instruction at an invalid position: {0}")]
    InvalidPatchPosition(usize)
}