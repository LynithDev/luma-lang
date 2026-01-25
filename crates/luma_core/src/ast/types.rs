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

    // special
    Ptr(Box<Type>),
    // Array(Box<Type>, Option<Spanned<usize>>),
    // Func(Vec<Type>, Box<Type>),
    Named(String),
}