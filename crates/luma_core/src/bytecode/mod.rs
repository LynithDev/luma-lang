use crate::{bytecode::chunk::{Chunk, FunctionChunk}};

pub mod opcode;
pub mod value;
pub mod chunk;

macro_rules! impl_wrapper_struct {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name($inner);

        impl $name {
            pub fn new(value: $inner) -> Self {
                Self(value)
            }
        }

        impl std::ops::Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, stringify!($name))?;
                write!(f, "(")?;
                write!(f, "{}", self.0)?;
                write!(f, ")")?;
                Ok(())
            }
        }
    };
}

impl_wrapper_struct!(IndexRef, usize);
impl_wrapper_struct!(ArityRef, u8);

pub mod prelude {
    pub use super::opcode::*;
    pub use super::value::*;
    pub use super::chunk::*;

    pub use super::{ArityRef, IndexRef, Bytecode};
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Bytecode {
    pub functions: Vec<FunctionChunk>,
    pub top_level: Chunk,
}

impl Bytecode {
    pub fn new() -> Self {
        Self::default()
    }
}