use luma_core::bytecode::prelude::*;

use crate::{frames::{CallFrame, ChunkRef}, slot_array::SlotArray, value::{HeapValue, StackValue}, LumaVM, VmError, VmExitCode, VmResult};

impl LumaVM {
    pub(super) fn exec(&mut self) -> VmResult<VmExitCode> {
        while let Ok(frame) = self.ctx.frames.last_mut() {
            let chunk = frame.try_get_chunk(&self.sources)?;

            if frame.instr_pointer >= chunk.instructions.len() {
                // reached the end of the chunk (and for whatever reason it didn't exit)
                self.ctx.frames.pop();
                continue;
            }

            let instruction = chunk.instructions[frame.instr_pointer].clone();
            frame.instr_pointer += 1;
            
            let opcode = instruction.opcode;

            println!("{}x{} exec opcode: {:?} at {}", frame.base, frame.instr_pointer, opcode, instruction.cursor);
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
            // OpCode::Jump(IndexRef),
            // OpCode::JumpIfFalse(IndexRef),
            
            // stack operations
            OpCode::GetLocal(index) => self.exec_get_local(index),
            OpCode::SetLocal(index) => self.exec_set_local(index),
            // OpCode::GetUpvalue(IndexRef),
            // OpCode::SetUpvalue(IndexRef),
            OpCode::Pop => self.exec_pop(),
            // OpCode::PopLocals(usize),
            
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
            .ok_or(VmError::IndexOutOfBounds(*const_index))?
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

    fn exec_call(&mut self, arity: ArityRef) -> VmResult<()> {
        let arg_count = *arity;

        // func is below args
        let func_index = match self.ctx.stack.pop()? {
            StackValue::HeapRef(index) => index,
            value => {
                return Err(VmError::InvalidType(value.to_string()));
            }
        };
        
        let frame = self.ctx.frames.last_mut()?;

        let value = frame.locals.try_get(*func_index)?;

        match value {
            StackValue::HeapRef(heap_index) => {
                let heap_value = self.ctx.heap.try_get(*heap_index)?;

                match heap_value {
                    HeapValue::Function(func_ref) => {
                        let source_index = func_ref.source_index;
                        let function_index = func_ref.function_index;

                        let source = &self.sources[*source_index];
                        let function_chunk = &source.bytecode.functions[*function_index];

                        if arg_count != *function_chunk.arity {
                            return Err(VmError::ArityMismatch(
                                *function_chunk.arity,
                                arg_count,
                            ));
                        }

                        let new_frame = CallFrame {
                            source_index,
                            chunk_ref: ChunkRef::Function(function_index),
                            instr_pointer: 0,
                            base: self.ctx.stack.len() - arg_count as usize,
                            locals: SlotArray::new(function_chunk.chunk.local_count),
                        };

                        self.ctx.frames.push(new_frame)?;
                    }
                    _ => {
                        return Err(VmError::InvalidType(value.to_string()));
                    }
                }
            }
            _ => {
                return Err(VmError::InvalidType(value.to_string()));
            }
        }

        Ok(())
    }

    fn exec_return(&mut self) -> VmResult<()> {
        let return_value = self.ctx.stack.pop();
        
        self.ctx.frames.pop();

        if let Ok(value) = return_value {
            self.ctx.stack.push(value)?;
        }

        Ok(())
    }

    fn exec_pop(&mut self) -> VmResult<()> {
        self.ctx.stack.pop()?;
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