use luma_core::bytecode::prelude::*;

use crate::{
    frames::{CallFrame, FrameSource, Upvalue}, slot_array::SlotArray, value::{Closure, HeapValue, StackValue}, LumaVM, VmError, VmExitCode, VmResult
};

impl LumaVM {
    pub(super) fn exec(&mut self) -> VmResult<VmExitCode> {
        loop {
            let frame_index = self.ctx.frames.len() - 1;
            let frame = match self.ctx.frames.get_mut(frame_index) {
                Ok(f) => f,
                Err(_) => break, // no more frames to execute
            };
            
            let chunk = frame.get_chunk();

            if frame.instr_pointer >= chunk.instructions.len() {
                dbg!(&self.ctx.stack);
                // reached the end of the chunk (and for whatever reason it didn't exit)
                self.pop_frame()?;
                break;
            }

            let instruction = chunk.instructions[frame.instr_pointer].clone();
            
            let opcode = instruction.opcode;
            // println!("{}x{} exec opcode: {:?}", frame_index, frame.instr_pointer, opcode);
            
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
            OpCode::Closure(const_index, local_index) => self.exec_closure(const_index, local_index),

            // flow control
            OpCode::Return => self.exec_return(),
            OpCode::Call(arity) => self.exec_call(arity),
            OpCode::Jump(index) => self.exec_jump(index),
            OpCode::JumpIfFalse(index) => self.exec_jump_if_false(index),

            // stack operations
            OpCode::Dup => self.exec_dup(),
            OpCode::GetLocal(index) => self.exec_get_local(index),
            OpCode::SetLocal(index) => self.exec_set_local(index),
            OpCode::GetUpvalue(index) => self.exec_get_upvalue(index),
            OpCode::SetUpvalue(index) => self.exec_set_upvalue(index),
            OpCode::Pop => self.exec_pop(),
            OpCode::PopLocals(amount) => self.exec_pop_n(amount),

            _ => {
                println!("unimplemented opcode {:?}", &opcode);

                Ok(())
            }
        }
    }

    fn exec_push_const(&mut self, const_index: IndexRef) -> VmResult<()> {
        let frame = self.ctx.frames.try_peek()?;

        let chunk = frame.get_chunk();
        let value = chunk
            .constants
            .get(*const_index)
            .ok_or(VmError::NoConstantAtIndex(*const_index))?
            .clone();

        self.alloc_value(value)?;

        Ok(())
    }

    fn exec_closure(&mut self, const_index: IndexRef, local_index: Option<IndexRef>) -> VmResult<()> {
        let frame = self.ctx.frames.try_peek()?;

        let chunk = frame.get_chunk();
        let value = chunk
            .constants
            .get(*const_index)
            .ok_or(VmError::NoConstantAtIndex(*const_index))?
            .clone();

        let func_index = match value {
            BytecodeValue::Function(index) => index,
            _ => return Err(VmError::TypeMismatch("Function".to_string(), format!("{:?}", value))),
        };

        let func_chunk = self.entrypoint().bytecode.functions.get(*func_index)
            .ok_or(VmError::NoFunctionAtIndex(*func_index))?;
        
        // create our closure object
        let closure = Closure {
            upvalues: SlotArray::new(func_chunk.upvalues.len()),
            function: func_chunk,
        };

        // pre-alloc our closure
        let ptr = self.ctx.closures.alloc(closure) as *mut Closure;
        
        let index = self.ctx.heap.push(HeapValue::Closure(ptr))?;

        // push our pre-allocated closure onto the stack
        let value = StackValue::HeapRef(index);
        if let Some(local_index) = local_index {
            self.set_local(local_index, value)?;
            local_index
        } else {
            self.ctx.stack.push(value)?
        };
        
        // now initialize upvalues
        let closure = unsafe { &mut *ptr };
        let func_chunk = unsafe { &*closure.function };

        for (i, desc) in func_chunk.upvalues.clone().iter().enumerate() {
            let upvalue = if desc.is_local {
                // points to a stack slot
                let value = self.get_local_mut(desc.index)?;

                if let StackValue::HeapRef(_) = value {
                    Upvalue::Closed(value.clone())
                } else {
                    Upvalue::Open(value as *mut StackValue)
                }
            } else {
                // points to an upvalue from the parent closure
                // let parent_upval = parent_closure.upvalues[parent_index];
                // parent_upval.clone()
                todo!("parent closure upvalue capture not implemented")
            };
            
            closure.upvalues.set(i, Some(upvalue))?;
        }

        Ok(())
    }

    fn exec_dup(&mut self) -> VmResult<()> {
        let value = self.ctx.stack.peek().unwrap_or(&StackValue::Unit);

        self.ctx.stack.push(value.clone())?;

        Ok(())
    }

    fn exec_set_local(&mut self, local_index: IndexRef) -> VmResult<()> {
        let value = self.ctx.stack.pop()?;

        self.set_local(local_index, value.clone())?;

        Ok(())
    }

    fn exec_get_local(&mut self, local_index: IndexRef) -> VmResult<()> {
        let value = self.get_local(local_index)?.clone();

        self.ctx.stack.push(value)?;

        Ok(())
    }

    fn exec_get_upvalue(&mut self, upvalue_index: IndexRef) -> VmResult<()> {
        let frame = self.ctx.frames.try_peek()?; // current frame

        let closure = match frame.source {
            FrameSource::Closure(closure_ptr) => unsafe { &*closure_ptr },
            _ => return Err(VmError::InvalidOperation("current frame is not a closure".to_string())),
        };

        let upvalue = closure.upvalues.try_get(*upvalue_index)?;

        let value = match upvalue {
            Upvalue::Open(ptr) => unsafe { (&**ptr).clone() },
            Upvalue::Closed(v) => v.clone(),
        };

        self.ctx.stack.push(value)?;

        Ok(())
    }

    fn exec_set_upvalue(&mut self, upvalue_index: IndexRef) -> VmResult<()> {
        let value = self.ctx.stack.pop()?;

        let frame = self.ctx.frames.try_peek()?;

        let closure = match frame.source {
            FrameSource::Closure(closure_ptr) => unsafe { &mut *closure_ptr },
            _ => return Err(VmError::InvalidOperation("current frame is not a closure".to_string())),
        };

        let uv = closure.upvalues.try_get_mut(*upvalue_index)?;
        match uv {
            Upvalue::Open(ptr) => unsafe { ptr.write(value.clone()) },
            Upvalue::Closed(slot) => *slot = value.clone(),
        }

        Ok(())
    }

    fn exec_call(&mut self, arity: ArityRef) -> VmResult<()> {
        // attempt to get the function value from the stack (well heap)
        let value = self.ctx.stack.pop()?;

        let StackValue::HeapRef(heap_index) = value else{
            return Err(VmError::InvalidType(format!("{:?}", value)))
        };

        let heap_val = self.ctx.heap.try_get(heap_index)?;
        let HeapValue::Closure(closure) = heap_val else {
            return Err(VmError::InvalidType(format!("{:?}", heap_val)))
        };

        let func_chunk = unsafe { &*(&**closure).function };

        // check arity
        if *arity != *func_chunk.arity {
            return Err(VmError::ArityMismatch(*func_chunk.arity, *arity));
        }

        // prepare new call frame
        let source = FrameSource::Closure(*closure as *mut _);

        let new_frame = CallFrame {
            source,
            instr_pointer: 0,
            base: self.ctx.stack.len() - *arity as usize,
        };

        let reserve_amount = func_chunk.chunk.local_count - *arity as usize;

        // println!("\ncalling function {:?} with arity {}", func_chunk.name, arity);
        // println!("{:#?}", &self.ctx.stack[new_frame.base..]);
        self.push_frame(new_frame, Some(reserve_amount))?;

        Ok(())
    }

    fn exec_jump(&mut self, index: IndexRef) -> VmResult<()> {
        let frame = self.ctx.frames.try_peek_mut()?;
        frame.instr_pointer = *index;
        Ok(())
    }

    fn exec_jump_if_false(&mut self, index: IndexRef) -> VmResult<()> {
        let condition = self.ctx.stack.pop()?;

        match condition {
            StackValue::Boolean(false) => {
                let frame = self.ctx.frames.try_peek_mut()?;
                frame.instr_pointer = *index;
            }
            StackValue::Boolean(true) => {
                // do nothing
            }
            _ => {
                return Err(VmError::TypeMismatch(
                    "Boolean".to_string(),
                    format!("{:?}", condition),
                ));
            }
        }

        Ok(())
    }

    fn exec_return(&mut self) -> VmResult<()> {
        let return_value = self.ctx.stack.pop().unwrap_or(StackValue::Unit);

        self.pop_frame()?;
        self.ctx.stack.push(return_value)?;

        Ok(())
    }

    fn exec_pop(&mut self) -> VmResult<()> {
        self.ctx.stack.pop()?;

        Ok(())
    }

    fn exec_pop_n(&mut self, n: usize) -> VmResult<()> {
        self.ctx.stack.pop_n(n)
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
            _ => Err(VmError::TypeMismatch(
                "Boolean".to_string(),
                format!("{:?}", value),
            )),
        }
    }

    fn exec_negate(&mut self) -> VmResult<()> {
        let value = self.ctx.stack.pop()?;

        self.ctx.stack.push((-value)?)?;

        Ok(())
    }
}
