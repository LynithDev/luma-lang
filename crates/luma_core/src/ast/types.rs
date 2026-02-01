use strum::Display;

use crate::MaybeSpanned;

pub type Type = MaybeSpanned<TypeKind>;

#[derive(Display, Debug, Clone, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum TypeKind {
    // primitive
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    Bool,
    Char,
    String,
    Tuple(Vec<Type>),
    Unit,

    // special
    Ptr(Box<Type>),
    // Array(Box<Type>, Option<Spanned<usize>>),
    // Func(Vec<Type>, Box<Type>),
    Named {
        name: String,
        def_id: Option<usize>,
    },
}

impl From<Type> for TypeKind {
    fn from(val: Type) -> Self {
        val.item
    }
}

impl TypeKind {
    #[must_use]
    pub const fn is_int(&self) -> bool {
        matches!(
            self,
            TypeKind::Int8 | TypeKind::Int16 | TypeKind::Int32 | TypeKind::Int64
        )
    }

    #[must_use]
    pub const fn is_uint(&self) -> bool {
        matches!(
            self,
            TypeKind::UInt8 | TypeKind::UInt16 | TypeKind::UInt32 | TypeKind::UInt64
        )
    }

    #[must_use]
    pub const fn is_float(&self) -> bool {
        matches!(self, TypeKind::Float32 | TypeKind::Float64)
    }

    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        self.is_int() || self.is_uint() || self.is_float()
    }

    #[must_use]
    pub const fn is_string(&self) -> bool {
        matches!(self, TypeKind::String)
    }

    #[must_use]
    pub const fn is_char(&self) -> bool {
        matches!(self, TypeKind::Char)
    }

    #[must_use]
    pub const fn is_ptr(&self) -> bool {
        matches!(self, TypeKind::Ptr(_))
    }

    #[must_use]
    pub const fn is_named(&self) -> bool {
        matches!(self, TypeKind::Named { .. })
    }

    #[must_use]
    pub const fn is_tuple(&self) -> bool {
        matches!(self, TypeKind::Tuple(_))
    }

    #[must_use]
    pub const fn is_bool(&self) -> bool {
        matches!(self, TypeKind::Bool)
    }

    pub fn bits(&self) -> Option<usize> {
        Some(match self {
            TypeKind::UInt8 | TypeKind::Int8 => 8,
            TypeKind::UInt16 | TypeKind::Int16 => 16,
            TypeKind::UInt32 | TypeKind::Int32 | TypeKind::Float32 => 32,
            TypeKind::UInt64 | TypeKind::Int64 | TypeKind::Float64 => 64,
            _ => return None,
        })
    }

    pub fn promote(left: &TypeKind, right: &TypeKind) -> Option<TypeKind> {
        use TypeKind::*;

        match (left, right) {
            (left, right) if left == right => Some(left.clone()),

            // float promotion
            (Float32, Float64) | (Float64, Float32) => Some(Float64),
            (Float32, _) | (_, Float32) => Some(Float32),
            (Float64, _) | (_, Float64) => Some(Float64),

            // signed integers
            (Int8, Int16) | (Int16, Int8) => Some(Int16),
            (Int8, Int32) | (Int32, Int8) => Some(Int32),
            (Int8, Int64) | (Int64, Int8) => Some(Int64),
            (Int16, Int32) | (Int32, Int16) => Some(Int32),
            (Int16, Int64) | (Int64, Int16) => Some(Int64),
            (Int32, Int64) | (Int64, Int32) => Some(Int64),

            // unsigned integers
            (UInt8, UInt16) | (UInt16, UInt8) => Some(UInt16),
            (UInt8, UInt32) | (UInt32, UInt8) => Some(UInt32),
            (UInt8, UInt64) | (UInt64, UInt8) => Some(UInt64),
            (UInt16, UInt32) | (UInt32, UInt16) => Some(UInt32),
            (UInt16, UInt64) | (UInt64, UInt16) => Some(UInt64),
            (UInt32, UInt64) | (UInt64, UInt32) => Some(UInt64),

            // signed vs unsigned -> promote to signed of largest size
            (Int8, UInt8) | (UInt8, Int8) => Some(Int16),
            (Int16, UInt16) | (UInt16, Int16) => Some(Int32),
            (Int32, UInt32) | (UInt32, Int32) => Some(Int64),
            
            // *risky*
            (Int64, UInt64) | (UInt64, Int64) => Some(Int64),

            // cant promote types
            _ => None,
        }
    }
}
