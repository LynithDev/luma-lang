mod expressions;
mod operators;
mod statements;
mod types;
mod visibility;

pub use expressions::*;
pub use operators::*;
pub use statements::*;
pub use types::*;
pub use visibility::*;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Ast {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalBranch {
    pub condition: Expression,
    pub body: Statement,
}
