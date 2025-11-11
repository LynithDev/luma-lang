use luma_core::bytecode::value::BytecodeValue;

use crate::{
    LumaVM, VmResult,
    value::{FunctionRef, HeapValue, StackValue},
};

impl LumaVM {
    pub(super) fn materialize_value(&mut self, value: BytecodeValue) -> VmResult<StackValue> {
        Ok(match value {
            // primitives get pushed to stack normally
            BytecodeValue::UInt8(i) => StackValue::UInt8(i),
            BytecodeValue::UInt16(i) => StackValue::UInt16(i),
            BytecodeValue::UInt32(i) => StackValue::UInt32(i),
            BytecodeValue::UInt64(i) => StackValue::UInt64(i),
            BytecodeValue::Int8(i) => StackValue::Int8(i),
            BytecodeValue::Int16(i) => StackValue::Int16(i),
            BytecodeValue::Int32(i) => StackValue::Int32(i),
            BytecodeValue::Int64(i) => StackValue::Int64(i),
            BytecodeValue::Float32(f) => StackValue::Float32(f),
            BytecodeValue::Float64(f) => StackValue::Float64(f),
            BytecodeValue::Boolean(b) => StackValue::Boolean(b),

            // heap-stored values get pushed to heap and return [`StackValue::HeapRef`]
            BytecodeValue::String(s) => {
                let index = self.ctx.heap.push(HeapValue::String(s))?;
                StackValue::HeapRef(index)
            }
            BytecodeValue::Function(func_index) => {
                let frame = self.ctx.frames.last_mut()?;

                let func_ref = FunctionRef {
                    source_index: frame.source_index,
                    function_index: func_index,
                };

                let index = self.ctx.heap.push(HeapValue::Function(func_ref))?;
                StackValue::HeapRef(index)
            }

            // todo: implement these
            BytecodeValue::Option(_) => unimplemented!("Option materialization not implemented"),
            BytecodeValue::NativeFunction(_) => {
                unimplemented!("NativeFunction materialization not implemented")
            }
        })
    }
}
