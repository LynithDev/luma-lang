use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub enum BytecodeValue {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Bool(bool),
    Char(char),
    String(String),
    Unit,
}

impl Eq for BytecodeValue {}

impl Hash for BytecodeValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            BytecodeValue::UInt8(v) => v.hash(state),
            BytecodeValue::UInt16(v) => v.hash(state),
            BytecodeValue::UInt32(v) => v.hash(state),
            BytecodeValue::UInt64(v) => v.hash(state),
            BytecodeValue::Int8(v) => v.hash(state),
            BytecodeValue::Int16(v) => v.hash(state),
            BytecodeValue::Int32(v) => v.hash(state),
            BytecodeValue::Int64(v) => v.hash(state),
            
            // todo: handle floats properly
            BytecodeValue::Float32(v) => v.to_bits().hash(state),
            BytecodeValue::Float64(v) => v.to_bits().hash(state),
            
            BytecodeValue::Bool(v) => v.hash(state),
            BytecodeValue::Char(v) => v.hash(state),
            BytecodeValue::String(v) => v.hash(state),

            BytecodeValue::Unit => state.write_u8(0),
        }
    }
}