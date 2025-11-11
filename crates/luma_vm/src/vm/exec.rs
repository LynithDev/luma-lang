use luma_core::bytecode::prelude::*;

use crate::{runtime::ChunkRef, value::StackValue, LumaVM, VmError, VmExitCode, VmResult};

impl LumaVM {
    pub(super) fn exec(&mut self) -> VmResult<VmExitCode> {
        while let Ok(frame) = self.ctx.frames.last_mut() {
            let chunk = match frame.chunk_ref {
                ChunkRef::TopLevel => &self.sources[*frame.source_index].bytecode.top_level,
                ChunkRef::Function(func_idx) => {
                    &self.sources[*frame.source_index].bytecode.functions[*func_idx].chunk
                }
            };

            if frame.instr_pointer >= chunk.instructions.len() {
                // reached the end of the chunk (and for whatever reason it didn't exit)
                self.ctx.frames.pop();
                continue;
            }

            let instruction = &chunk.instructions[frame.instr_pointer];
            frame.instr_pointer += 1;

            println!("opcode: {:?}", &instruction.opcode);
            match instruction.opcode {
                OpCode::Const(index) => self.push_const(index)?,
                OpCode::SetLocal(index) => self.set_local(index)?,
                OpCode::GetLocal(index) => self.get_local(index)?,
                OpCode::Add => self.add()?,
                _ => {}
            }
        }
        
        dbg!(&self.ctx.stack);
        dbg!(&self.ctx.heap);

        Ok(0)
    }

    fn push_const(&mut self, const_index: IndexRef) -> VmResult<()> {
        let frame = self
            .ctx
            .frames
            .last_mut()?;

        let source = &self.sources[*frame.source_index];
        let value = match frame.chunk_ref {
            ChunkRef::TopLevel => source.bytecode.top_level.constants[*const_index].clone(),
            ChunkRef::Function(func_idx) => {
                source.bytecode.functions[*func_idx].chunk.constants[*const_index].clone()
            }
        };

        let value = self.materialize_value(value)?;
        self.ctx.stack.push(value)?;

        Ok(())
    }

    fn set_local(&mut self, local_index: IndexRef) -> VmResult<()> {
        let value = self.ctx.stack.pop()?;

        let frame = self
            .ctx
            .frames
            .last_mut()?;

        self.ctx.stack.set_local(frame.base, *local_index, value)?;

        Ok(())
    }

    fn get_local(&mut self, local_index: IndexRef) -> VmResult<()> {
        let frame = self
            .ctx
            .frames
            .last_mut()?;

        let value = self.ctx.stack.get_local(frame.base, *local_index)?.clone();

        self.ctx.stack.push(value)?;

        Ok(())
    }

    fn add(&mut self) -> VmResult<()> {
        let right = self.ctx.stack.pop()?;
        let left = self.ctx.stack.pop()?;

        let value = match (left, right) {
            (StackValue::UInt8(l), StackValue::UInt8(r)) => StackValue::UInt8(l.wrapping_add(r)),
            (StackValue::UInt16(l), StackValue::UInt16(r)) => StackValue::UInt16(l.wrapping_add(r)),
            (StackValue::UInt32(l), StackValue::UInt32(r)) => StackValue::UInt32(l.wrapping_add(r)),
            (StackValue::UInt64(l), StackValue::UInt64(r)) => StackValue::UInt64(l.wrapping_add(r)),
            (StackValue::Int8(l), StackValue::Int8(r)) => StackValue::Int8(l.wrapping_add(r)),
            (StackValue::Int16(l), StackValue::Int16(r)) => StackValue::Int16(l.wrapping_add(r)),
            (StackValue::Int32(l), StackValue::Int32(r)) => StackValue::Int32(l.wrapping_add(r)),
            (StackValue::Int64(l), StackValue::Int64(r)) => StackValue::Int64(l.wrapping_add(r)),
            (StackValue::Float32(l), StackValue::Float32(r)) => StackValue::Float32(Float32(*l + *r)),
            (StackValue::Float64(l), StackValue::Float64(r)) => StackValue::Float64(Float64(*l + *r)),
            _ => return Err(VmError::TypeMismatch),
        };

        self.ctx.stack.push(value)?;

        Ok(())
    }
}
