use thiserror::Error;

use crate::stages::codegen::chunk::{ConstSlot, LocalSlot};

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CodegenErrorKind {
    #[error("attempted to patch an instruction at an invalid position: {0}")]
    InvalidPatchPosition(usize),
    #[error("too many local variables declared in a single chunk (max {})", LocalSlot::MAX)]
    TooManyLocals,
    #[error("too many constants in a single chunk (max {})", ConstSlot::MAX)]
    TooManyConstants,
    #[error("undefined local variable with symbol id {0}")]
    UndefinedLocal(usize),
}