use luma_core::bytecode::prelude::*;

use crate::{frames::{CallFrame, ChunkRef, Upvalue}, slot_array::SlotArray, value::{HeapValue, StackValue}, LumaVM, VmError, VmExitCode, VmResult};

impl LumaVM {
    pub(super) fn exec(&mut self) -> VmResult<VmExitCode> {
        while let Ok(frame) = self.ctx.frames.last_mut() {
            let chunk = frame.try_get_chunk(&self.sources)?;
            
            if frame.instr_pointer >= chunk.instructions.len() {
                // reached the end of the chunk (and for whatever reason it didn't exit)
                dbg!(&self.ctx);
                self.ctx.frames.pop();
                continue;
            }

            let instruction = chunk.instructions[frame.instr_pointer].clone();
            
            let opcode = instruction.opcode;
            println!("{}x{} exec opcode: {:?} at {}", frame.base, frame.instr_pointer, opcode, instruction.cursor);
            frame.instr_pointer += 1;

            if let Err(err) = self.exec_opcode(opcode) {
                eprintln!("error at {}: {}", instruction.cursor, err);
                return Err(err);
            };
        }


        Ok(0)
    }

    pub(super) fn exec_opcode(&mut self, opcode: OpCode) -> VmResult<()> {
        match opcode {
            // binary operators
            OpCode::Add => self.exec_add(),
            OpCode::Sub => self.exec_sub(),
            OpCode::Mul => self.exec_mul(),
            OpCode::Div => self.exec_div(),
            OpCode::Mod => self.exec_modulo(),
            OpCode::BitAnd => self.exec_bit_and(),
            OpCode::BitOr => self.exec_bit_or(),
            OpCode::BitXor => self.exec_bit_xor(),
            OpCode::ShiftLeft => self.exec_shift_left(),
            OpCode::ShiftRight => self.exec_shift_right(),

            // comparison operators
            OpCode::Equal => self.exec_equal(),
            OpCode::GreaterThan => self.exec_greater(),
            OpCode::GreaterThanEqual => self.exec_greater_equal(),
            OpCode::LesserThan => self.exec_lesser(),
            OpCode::LesserThanEqual => self.exec_lesser_equal(),
            OpCode::NotEqual => self.exec_not_equal(),

            // logical operators
            // OpCode::And => self.and(),
            // OpCode::Or => self.or(),
            OpCode::Negate => self.exec_negate(),
            OpCode::Not => self.exec_not(),
            // OpCode::BitNot => self.not(),

            // literals
            OpCode::Const(index) => self.exec_push_const(index),
            
            // flow control
            OpCode::Return => self.exec_return(),
            OpCode::Call(arity) => self.exec_call(arity),
            OpCode::Jump(index) => self.exec_jump(index),
            OpCode::JumpIfFalse(index) => self.exec_jump_if_false(index),
            
            // stack operations
            OpCode::GetLocal(index) => self.exec_get_local(index),
            OpCode::SetLocal(index) => self.exec_set_local(index),
            OpCode::GetUpvalue(index) => self.exec_get_upvalue(index),
            OpCode::SetUpvalue(index) => self.exec_set_upvalue(index),
            OpCode::Pop => self.exec_pop(),
            OpCode::PopLocals(amount) => self.exec_pop_locals(amount),
            
            _ => {
                println!("unimplemented opcode {:?}", &opcode);

                Ok(())
            },
        }
    }

    fn exec_push_const(&mut self, const_index: IndexRef) -> VmResult<()> {
        let frame = self
            .ctx
            .frames
            .last_mut()?;

        let chunk = frame.try_get_chunk(&self.sources)?;
        let value = chunk
            .constants
            .get(*const_index)
            .ok_or(VmError::NoConstantAtIndex(*const_index))?
            .clone();

        let value = self.materialize_value(value)?;
        self.ctx.stack.push(value)?;

        Ok(())
    }

    fn exec_set_local(&mut self, local_index: IndexRef) -> VmResult<()> {
        let value = self.ctx.stack.pop()?;

        let frame = self
            .ctx
            .frames
            .last_mut()?;

        frame.locals.set(*local_index, Some(value))?;

        Ok(())
    }

    fn exec_get_local(&mut self, local_index: IndexRef) -> VmResult<()> {
        let frame = self
            .ctx
            .frames
            .last_mut()?;

        let value = frame.locals.try_get(*local_index)?.clone();

        self.ctx.stack.push(value)?;

        Ok(())
    }

    fn exec_get_upvalue(&mut self, upvalue_index: IndexRef) -> VmResult<()> {
        let frame = self.ctx.frames.last_mut()?; // current frame
        let upvalue = frame.upvalues.try_get(*upvalue_index)?.clone();

        let value = match upvalue {
            Upvalue::Open(ptr) => unsafe { (*ptr).clone() },
            Upvalue::Closed(v) => v,
        };

        self.ctx.stack.push(value)?;
        Ok(())
    }

    fn exec_set_upvalue(&mut self, upvalue_index: IndexRef) -> VmResult<()> {
        let mut value = self.ctx.stack.pop()?;
        let frame = self.ctx.frames.last_mut()?;
        let uv = frame.upvalues.try_get_mut(*upvalue_index)?;

        match uv {
            Upvalue::Open(ptr) => *ptr = &mut value,
            Upvalue::Closed(slot) => *slot = value,
        }

        Ok(())
    }

    fn exec_call(&mut self, arity: ArityRef) -> VmResult<()> {
        let arg_count = *arity;

        // func is below args
        let func_value = self.ctx.stack.pop()?;

        let func_ref = match func_value {
            StackValue::HeapRef(heap_index) => {
                let heap_val = self.ctx.heap.try_get(heap_index)?;
                match heap_val {
                    HeapValue::Function(func_ref) => func_ref,
                    _ => return Err(VmError::InvalidType(format!("{:?}", heap_val))),
                }
            }
            _ => return Err(VmError::InvalidType(format!("{:?}", func_value))),
        };
        
        let source = &self.sources[*func_ref.source_index];
        let func_chunk = &source.bytecode.functions[*func_ref.function_index];

        if arg_count != *func_chunk.arity {
            return Err(VmError::ArityMismatch(*func_chunk.arity, arg_count));
        }

        let parent_frame = self.ctx.frames.last_mut()?;

        // initialize upvalues for the new frame
        let mut upvalues = SlotArray::new(func_chunk.upvalues.len());
        for (i, desc) in func_chunk.upvalues.iter().enumerate() {

            let uv = if desc.is_local {
                let ptr = parent_frame
                    .locals
                    .get_mut(*desc.index)
                    .ok_or(VmError::NoLocalAtIndex(*desc.index))? as *mut StackValue;

                Upvalue::Open(ptr)
            } else {
                // capture parent's upvalue
                parent_frame.upvalues.try_get(*desc.index)?.clone()
            };

            upvalues.set(i, Some(uv))?;
        }

        let new_frame = CallFrame {
            source_index: func_ref.source_index,
            chunk_ref: ChunkRef::Function(func_ref.function_index),
            instr_pointer: 0,
            base: self.ctx.stack.len() - arg_count as usize,
            locals: SlotArray::new(func_chunk.chunk.local_count),
            upvalues,
        };

        self.ctx.frames.push(new_frame)?;
        Ok(())
    }

    fn exec_jump(&mut self, index: IndexRef) -> VmResult<()> {
        let frame = self.ctx.frames.last_mut()?;
        frame.instr_pointer = *index;
        Ok(())
    }

    fn exec_jump_if_false(&mut self, index: IndexRef) -> VmResult<()> {
        let condition = self.ctx.stack.pop()?;

        match condition {
            StackValue::Boolean(false) => {
                let frame: &mut CallFrame = self.ctx.frames.last_mut()?;
                frame.instr_pointer = *index;
            }
            StackValue::Boolean(true) => {
                // do nothing
            }
            _ => return Err(VmError::TypeMismatch("Boolean".to_string(), format!("{:?}", condition))),
        }

        Ok(())
    }

    fn exec_return(&mut self) -> VmResult<()> {
        let return_value = self.ctx.stack.pop().unwrap_or(StackValue::Unit);
        
        self.ctx.frames.pop();
        self.ctx.stack.push(return_value)?;

        Ok(())
    }

    fn exec_pop(&mut self) -> VmResult<()> {
        self.ctx.stack.pop()?;
        Ok(())
    }

    fn exec_pop_locals(&mut self, amount: usize) -> VmResult<()> {
        let frame = self.ctx.frames.last_mut()?;

        frame.locals.clear_range(frame.locals.len() - amount, frame.locals.len())?;

        Ok(())
    }

}

macro_rules! impl_bin_op {
    ($name:ident, $fn_name:ident) => {
        fn $name(&mut self) -> VmResult<()> {
            let right = self.ctx.stack.pop()?;
            let left = self.ctx.stack.pop()?;

            use std::ops::*;
            let value = (left.$fn_name(right))?;

            self.ctx.stack.push(value)?;

            Ok(())
        }
    };
}

macro_rules! impl_cmp_op {
    ($name:ident, $fn_name:ident) => {
        fn $name(&mut self) -> VmResult<()> {
            let right = self.ctx.stack.pop()?;
            let left = self.ctx.stack.pop()?;

            use core::cmp::*;
            let value = (left.$fn_name(&right));

            self.ctx.stack.push(StackValue::Boolean(value))?;

            Ok(())
        }
    };
}

#[allow(unused)]
impl LumaVM {
    impl_bin_op!(exec_add, add);
    impl_bin_op!(exec_sub, sub);
    impl_bin_op!(exec_mul, mul);
    impl_bin_op!(exec_div, div);
    impl_bin_op!(exec_modulo, rem);
    impl_bin_op!(exec_bit_and, bitand);
    impl_bin_op!(exec_bit_or, bitor);
    impl_bin_op!(exec_bit_xor, bitxor);
    impl_bin_op!(exec_shift_left, shl);
    impl_bin_op!(exec_shift_right, shr);

    impl_cmp_op!(exec_greater_equal, ge);
    impl_cmp_op!(exec_greater, gt);
    impl_cmp_op!(exec_lesser_equal, le);
    impl_cmp_op!(exec_lesser, lt);
    impl_cmp_op!(exec_equal, eq);
    impl_cmp_op!(exec_not_equal, ne);

    fn exec_not(&mut self) -> VmResult<()> {
        let value = self.ctx.stack.pop()?;

        match value {
            StackValue::Boolean(b) => {
                self.ctx.stack.push(StackValue::Boolean(!b))?;
                Ok(())
            }
            _ => Err(VmError::TypeMismatch("Boolean".to_string(), format!("{:?}", value))),
        }
    }

    fn exec_negate(&mut self) -> VmResult<()> {
        let value = self.ctx.stack.pop()?;

        self.ctx.stack.push((-value)?)?;

        Ok(())
    }
}