use crate::{Cursor, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
    pub cursor: Cursor,
}

pub type TypeRef = std::rc::Rc<std::cell::RefCell<Type>>;

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
        params: Vec<TypeKind>,
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
