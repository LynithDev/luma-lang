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
                OpCode::Sub => self.sub()?,
                OpCode::Mul => self.mul()?,
                OpCode::Div => self.div()?,
                OpCode::Mod => self.modulo()?,
                OpCode::BitAnd => self.bit_and()?,
                OpCode::BitOr => self.bit_or()?,
                OpCode::BitXor => self.bit_xor()?,
                OpCode::ShiftLeft => self.shift_left()?,
                OpCode::ShiftRight => self.shift_right()?,
                OpCode::Negate => self.negate()?,
                OpCode::Not => self.not()?,
                OpCode::GreaterThanEqual => self.greater_equal()?,
                OpCode::GreaterThan => self.greater()?,
                OpCode::LesserThanEqual => self.lesser_equal()?,
                OpCode::LesserThan => self.lesser()?,
                OpCode::Equal => self.equal()?,
                OpCode::NotEqual => self.not_equal()?,
                
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
    impl_bin_op!(add, add);
    impl_bin_op!(sub, sub);
    impl_bin_op!(mul, mul);
    impl_bin_op!(div, div);
    impl_bin_op!(modulo, rem);
    impl_bin_op!(bit_and, bitand);
    impl_bin_op!(bit_or, bitor);
    impl_bin_op!(bit_xor, bitxor);
    impl_bin_op!(shift_left, shl);
    impl_bin_op!(shift_right, shr);

    impl_cmp_op!(greater_equal, ge);
    impl_cmp_op!(greater, gt);
    impl_cmp_op!(lesser_equal, le);
    impl_cmp_op!(lesser, lt);
    impl_cmp_op!(equal, eq);
    impl_cmp_op!(not_equal, ne);

    fn not(&mut self) -> VmResult<()> {
        let value = self.ctx.stack.pop()?;

        match value {
            StackValue::Boolean(b) => {
                self.ctx.stack.push(StackValue::Boolean(!b))?;
                Ok(())
            }
            _ => Err(VmError::TypeMismatch),
        }
    }

    fn negate(&mut self) -> VmResult<()> {
        let value = self.ctx.stack.pop()?;

        self.ctx.stack.push((-value)?)?;

        Ok(())
    }
}