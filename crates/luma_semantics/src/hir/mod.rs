pub mod expressions;
pub mod statements;

pub mod prelude {
    pub use super::expressions::*;
    pub use super::statements::*;

    pub use luma_core::types::*;
    pub use luma_core::operators::*;
    pub use luma_core::visibility::*;

    pub use super::{Hir, HirConditionalBranch};
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Hir {
    pub statements: Vec<prelude::HirStatement>,
}

impl Hir {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { statements: Vec::with_capacity(capacity) }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HirConditionalBranch {
    pub condition: prelude::HirExpression,
    pub body: prelude::HirStatement,
}