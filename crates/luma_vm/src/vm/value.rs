use std::{hash::Hash, ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub}, rc::Rc};

use luma_core::{bytecode::{chunk::FunctionChunk, value::{Float32, Float64}, IndexRef}, Display};

use crate::{frames::Upvalue, slot_array::SlotArray, VmResult};

#[derive(Display, Debug, Clone, PartialEq, Eq, Hash)]
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
    Unit,

    HeapRef(IndexRef),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HeapValue {
    String(Rc<String>),
    Closure(*mut Closure),
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub function: *const FunctionChunk,
    pub upvalues: SlotArray<Upvalue>,
}

impl PartialEq for Closure {
    fn eq(&self, other: &Self) -> bool {
        self.function == other.function
    }
}

impl Eq for Closure {}

impl Hash for Closure {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.function.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionRef {
    pub source_index: IndexRef,
    pub function_index: IndexRef,
}

macro_rules! impl_op {
    ($op_name:ident, $fn_name:ident, $method_name:ident, $float_supported:tt, $float_op:tt) => {
        impl $op_name for StackValue {
            type Output = VmResult<Self>;

            fn $fn_name(self, rhs: Self) -> Self::Output {
                impl_op!(@float_case $float_supported, self, rhs, $float_op);

                match (&self, &rhs) {
                    (&StackValue::UInt8(l), &StackValue::UInt8(r)) => Ok(StackValue::UInt8(l.$method_name(r))),
                    (&StackValue::UInt16(l), &StackValue::UInt16(r)) => Ok(StackValue::UInt16(l.$method_name(r))),
                    (&StackValue::UInt32(l), &StackValue::UInt32(r)) => Ok(StackValue::UInt32(l.$method_name(r))),
                    (&StackValue::UInt64(l), &StackValue::UInt64(r)) => Ok(StackValue::UInt64(l.$method_name(r))),
                    (&StackValue::Int8(l), &StackValue::Int8(r)) => Ok(StackValue::Int8(l.$method_name(r))),
                    (&StackValue::Int16(l), &StackValue::Int16(r)) => Ok(StackValue::Int16(l.$method_name(r))),
                    (&StackValue::Int32(l), &StackValue::Int32(r)) => Ok(StackValue::Int32(l.$method_name(r))),
                    (&StackValue::Int64(l), &StackValue::Int64(r)) => Ok(StackValue::Int64(l.$method_name(r))),

                    _ => Err(crate::VmError::TypeMismatch(self.to_string(), rhs.to_string())),
                }
            }
        }
    };

    (@float_case true, $self:ident, $rhs:ident, $float_op:tt) => {
        match (&$self, &$rhs) {
            (StackValue::Float32(l), StackValue::Float32(r)) => return Ok(StackValue::Float32(Float32(l.0 $float_op r.0))),
            (StackValue::Float64(l), StackValue::Float64(r)) => return Ok(StackValue::Float64(Float64(l.0 $float_op r.0))),
            _ => {}
        }
    };

    (@float_case false, $self:ident, $rhs:ident, $float_op:tt) => {
        // no-op
    };
}

// arithmetic
impl_op!(Add, add, wrapping_add, true, +);
impl_op!(Sub, sub, wrapping_sub, true, -);
impl_op!(Mul, mul, wrapping_mul, true, *);
impl_op!(Div, div, wrapping_div, true, /);
impl_op!(Rem, rem, wrapping_rem, true, %);

// bitwise
impl_op!(Shl, shl, shl, false, <<);
impl_op!(Shr, shr, shr, false, >>);
impl_op!(BitAnd, bitand, bitand, false, &);
impl_op!(BitOr, bitor, bitor, false, |);
impl_op!(BitXor, bitxor, bitxor, false, ^);

impl PartialOrd for StackValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::UInt8(l0), Self::UInt8(r0)) => l0.partial_cmp(r0),
            (Self::UInt16(l0), Self::UInt16(r0)) => l0.partial_cmp(r0),
            (Self::UInt32(l0), Self::UInt32(r0)) => l0.partial_cmp(r0),
            (Self::UInt64(l0), Self::UInt64(r0)) => l0.partial_cmp(r0),
            (Self::Int8(l0), Self::Int8(r0)) => l0.partial_cmp(r0),
            (Self::Int16(l0), Self::Int16(r0)) => l0.partial_cmp(r0),
            (Self::Int32(l0), Self::Int32(r0)) => l0.partial_cmp(r0),
            (Self::Int64(l0), Self::Int64(r0)) => l0.partial_cmp(r0),
            (Self::Float32(l0), Self::Float32(r0)) => l0.partial_cmp(r0),
            (Self::Float64(l0), Self::Float64(r0)) => l0.partial_cmp(r0),
            (Self::HeapRef(l0), Self::HeapRef(r0)) => l0.partial_cmp(r0),
            _ => None,
        }
    }
}

impl Neg for StackValue {
    type Output = VmResult<Self>;

    fn neg(self) -> Self::Output {
        match self {
            StackValue::Int8(v) => Ok(StackValue::Int8(v.wrapping_neg())),
            StackValue::Int16(v) => Ok(StackValue::Int16(v.wrapping_neg())),
            StackValue::Int32(v) => Ok(StackValue::Int32(v.wrapping_neg())),
            StackValue::Int64(v) => Ok(StackValue::Int64(v.wrapping_neg())),
            StackValue::Float32(v) => Ok(StackValue::Float32(Float32(-v.0))),
            StackValue::Float64(v) => Ok(StackValue::Float64(Float64(-v.0))),
            _ => Err(crate::VmError::InvalidType(self.to_string())),
        }
    }
}

impl Not for StackValue {
    type Output = VmResult<Self>;

    fn not(self) -> Self::Output {
        match self {
            StackValue::Boolean(v) => Ok(StackValue::Boolean(!v)),
            _ => Err(crate::VmError::InvalidType(self.to_string())),
        }
    }
}