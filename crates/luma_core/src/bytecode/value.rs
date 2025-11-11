use std::{fmt::Display, hash::Hash};

use crate::bytecode::IndexRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BytecodeValue {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(Float32),
    Float64(Float64),
    Boolean(bool),
    String(String),
    Option(Box<Option<BytecodeValue>>),
    Function(IndexRef),
    NativeFunction(*const u8),
}

impl Display for BytecodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytecodeValue::UInt8(v) => write!(f, "{}", v),
            BytecodeValue::UInt16(v) => write!(f, "{}", v),
            BytecodeValue::UInt32(v) => write!(f, "{}", v),
            BytecodeValue::UInt64(v) => write!(f, "{}", v),
            BytecodeValue::Int8(v) => write!(f, "{}", v),
            BytecodeValue::Int16(v) => write!(f, "{}", v),
            BytecodeValue::Int32(v) => write!(f, "{}", v),
            BytecodeValue::Int64(v) => write!(f, "{}", v),
            BytecodeValue::Float32(v) => write!(f, "{}", v),
            BytecodeValue::Float64(v) => write!(f, "{}", v),
            BytecodeValue::Boolean(v) => write!(f, "{}", v),
            BytecodeValue::String(v) => write!(f, "\"{}\"", v),
            BytecodeValue::Option(v) => match &**v {
                Some(inner) => write!(f, "Some({})", inner),
                None => write!(f, "None"),
            },
            BytecodeValue::Function(index) => write!(f, "<Fn@{}>", index),
            BytecodeValue::NativeFunction(ptr) => write!(f, "<NFn@{:p}>", ptr),
        }
    }
}

macro_rules! impl_float_type {
    ($name:ident, $ty:ty, $bits:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name(pub $ty);

        impl $name {
            pub fn to_bits(&self) -> $bits {
                self.0.to_bits()
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.to_bits() == other.to_bits()
            }
        }
        
        impl Eq for $name {}

        impl Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.to_bits().hash(state);
            }
        }

        impl std::ops::Deref for $name {
            type Target = $ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}


impl_float_type!(Float32, f32, u32);
impl_float_type!(Float64, f64, u64);
