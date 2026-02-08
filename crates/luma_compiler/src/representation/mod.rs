pub mod ast;
pub mod aast;
pub mod bytecode;

mod operator;
mod visibility;
mod types;

pub use operator::{Operator, OperatorKind};
pub use visibility::{Visibility, VisibilityKind};
pub use types::{Type, TypeKind};

pub type ScopeId = usize;
pub type SymbolId = usize;
