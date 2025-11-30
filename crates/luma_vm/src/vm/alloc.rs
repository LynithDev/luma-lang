use luma_core::bytecode::{IndexRef, value::BytecodeValue};

use crate::{
    value::{HeapValue, StackValue}, LumaVM, VmError, VmResult
};

impl LumaVM {
    pub(super) fn alloc_value(&mut self, value: BytecodeValue) -> VmResult<IndexRef> {
        let value = match value {
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
            BytecodeValue::Unit => StackValue::Unit,

            // heap-stored values get pushed to heap and return [`StackValue::HeapRef`]
            BytecodeValue::String(s) => {
                let index = self.ctx.heap.push(HeapValue::String(s))?;
                StackValue::HeapRef(index)
            }
            BytecodeValue::Function(_) => {
                return Err(VmError::InvalidOperation("function allocations should be done with CLOSURE opcode".to_string()))
            }

            // todo: implement these
            BytecodeValue::Option(_) => unimplemented!("Option materialization not implemented"),
            BytecodeValue::NativeFunction(_) => {
                unimplemented!("NativeFunction materialization not implemented")
            }
        };

        self.ctx.stack.push(value)
    }

    pub fn set_local(&mut self, index: IndexRef, value: Option<StackValue>) -> VmResult<IndexRef> {
        let frame = self.ctx.frames.last()?;

        self.ctx.stack.set(frame.base + *index, value)?;
        Ok(IndexRef::new(frame.base + *index))
    }

    pub fn get_local(&self, index: IndexRef) -> VmResult<&StackValue> {
        let frame = self.ctx.frames.last()?;

        self.ctx.stack.get(frame.base + *index)
    }

    pub fn get_local_mut(&mut self, index: IndexRef) -> VmResult<&mut StackValue> {
        let frame = self.ctx.frames.last()?;

        self.ctx.stack.get_mut(frame.base + *index)
    }
}
