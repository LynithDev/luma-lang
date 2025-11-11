use luma_core::bytecode::{value::{Float32, Float64}, IndexRef};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StackValue {
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

    HeapRef(IndexRef),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HeapValue {
    String(String),
    Function(FunctionRef),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionRef {
    pub source_index: IndexRef,
    pub function_index: IndexRef,
}
