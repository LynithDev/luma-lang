use crate::{Cursor, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
    pub cursor: Cursor,
}

#[derive(crate::Display, Debug, Clone, PartialEq, Eq)]
#[display(case = "lowercase")]
pub enum TypeKind {
    String,
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
    Boolean,
    Void,
    Function {
        param_types: Vec<TypeKind>,
        return_type: Box<TypeKind>,
    },
    Array(Box<TypeKind>),
    Object(String),
}

impl From<&str> for TypeKind {
    fn from(value: &str) -> Self {
        match value {
            "string" => TypeKind::String,
            
            "u8" => TypeKind::UInt8,
            "u16" => TypeKind::UInt16,
            "u32" => TypeKind::UInt32,
            "u64" => TypeKind::UInt64,

            "i8" => TypeKind::Int8,
            "i16" => TypeKind::Int16,
            "i32" => TypeKind::Int32,
            "i64" => TypeKind::Int64,
            #[cfg(target_pointer_width = "32")]
            "int" => TypeKind::Int32,
            #[cfg(target_pointer_width = "64")]
            "int" => TypeKind::Int64,
            
            "f32" => TypeKind::Float32,
            "f64" => TypeKind::Float64,
            #[cfg(target_pointer_width = "32")]
            "float" => TypeKind::Float32,
            #[cfg(target_pointer_width = "64")]
            "float" => TypeKind::Float64,
            
            "bool" => TypeKind::Boolean,
            "void" => TypeKind::Void,
            
            _ if value.starts_with("fn") => {
                todo!("function type parsing is not yet implemented");
            }
            _ if value.ends_with("[]") => {
                let inner_type = Self::from(&value[..value.len() - 2]);
                TypeKind::Array(Box::new(inner_type))
            }
            _ => TypeKind::Object(value.to_string()),
        }
    }
}

impl TypeKind {
    pub fn precedence(&self) -> u8 {
        // lower value means higher precedence
        // e.g. during type coercion, the type with the lower value will be chosen
        match self {
            TypeKind::Void => 0,
            TypeKind::String => 1,
            TypeKind::Array(_) => 2,

            TypeKind::Float64 => 3,
            TypeKind::Float32 => 4,

            TypeKind::Int64 => 5,
            TypeKind::Int32 => 6,
            TypeKind::Int16 => 7,
            TypeKind::Int8 => 8,

            TypeKind::UInt64 => 9,
            TypeKind::UInt32 => 10,
            TypeKind::UInt16 => 11,
            TypeKind::UInt8 => 12,

            TypeKind::Boolean => 13,

            TypeKind::Object(_) => 14,
            TypeKind::Function { .. } => 15,
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            TypeKind::UInt8
                | TypeKind::UInt16
                | TypeKind::UInt32
                | TypeKind::UInt64
                | TypeKind::Int8
                | TypeKind::Int16
                | TypeKind::Int32
                | TypeKind::Int64
                | TypeKind::Float32
                | TypeKind::Float64
        )
    }

    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            TypeKind::UInt8
                | TypeKind::UInt16
                | TypeKind::UInt32
                | TypeKind::UInt64
                | TypeKind::Int8
                | TypeKind::Int16
                | TypeKind::Int32
                | TypeKind::Int64
        )
    }

    pub fn is_floating_point(&self) -> bool {
        matches!(self, TypeKind::Float32 | TypeKind::Float64)
    }

    pub fn is_signed(&self) -> bool {
        matches!(
            self,
            TypeKind::Int8
                | TypeKind::Int16
                | TypeKind::Int32
                | TypeKind::Int64
                | TypeKind::Float32
                | TypeKind::Float64
        )
    }

    pub fn is_unsigned(&self) -> bool {
        matches!(
            self,
            TypeKind::UInt8 | TypeKind::UInt16 | TypeKind::UInt32 | TypeKind::UInt64
        )
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, TypeKind::Boolean)
    }

    pub fn is_array(&self) -> bool {
        matches!(self, TypeKind::Array(_))
    }

    pub fn as_signed(&self) -> TypeKind {
        match self {
            TypeKind::UInt8 => TypeKind::Int8,
            TypeKind::UInt16 => TypeKind::Int16,
            TypeKind::UInt32 => TypeKind::Int32,
            TypeKind::UInt64 => TypeKind::Int64,
            _ => self.clone(),
        }
    }

    pub fn as_unsigned(&self) -> TypeKind {
        match self {
            TypeKind::Int8 => TypeKind::UInt8,
            TypeKind::Int16 => TypeKind::UInt16,
            TypeKind::Int32 => TypeKind::UInt32,
            TypeKind::Int64 => TypeKind::UInt64,
            _ => self.clone(),
        }
    }

    pub fn bit_size(&self) -> Option<usize> {
        match self {
            TypeKind::UInt8 | TypeKind::Int8 => Some(8),
            TypeKind::UInt16 | TypeKind::Int16 => Some(16),
            TypeKind::UInt32 | TypeKind::Int32 | TypeKind::Float32 => Some(32),
            TypeKind::UInt64 | TypeKind::Int64 | TypeKind::Float64 => Some(64),
            _ => None,
        }
    }
}