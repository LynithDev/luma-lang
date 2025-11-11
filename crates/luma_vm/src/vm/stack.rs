use std::fmt::Debug;

use luma_core::bytecode::IndexRef;

use crate::{VmError, VmResult, value::StackValue};

pub struct Stack {
    inner: Box<[Option<StackValue>]>,
    top: usize,
}

impl Stack {
    pub fn new(len: usize) -> Self {
        Self {
            inner: vec![None; len].into_boxed_slice(),
            top: 0,
        }
    }

    pub fn push(&mut self, value: StackValue) -> VmResult<IndexRef> {
        let index = self.top;
        if index >= self.inner.len() {
            return Err(VmError::StackOverflow);
        }

        self.inner[self.top] = Some(value);
        self.top += 1;

        Ok(IndexRef::new(index))
    }

    pub fn pop(&mut self) -> VmResult<StackValue> {
        if self.top == 0 {
            return Err(VmError::StackUnderflow);
        }

        self.top -= 1;
        self.inner[self.top].take().ok_or(VmError::NullReference)
    }

    pub fn peek(&self) -> Option<&StackValue> {
        if self.top == 0 {
            None
        } else {
            self.inner[self.top - 1].as_ref()
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.top
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    pub fn total_alloc_size(&self) -> usize {
        std::mem::size_of::<Stack>() * self.capacity()
    }

    pub fn set_local(
        &mut self,
        frame_base: usize,
        local_index: usize,
        value: StackValue,
    ) -> VmResult<()> {
        let idx = frame_base + local_index;
        if idx >= self.inner.len() {
            return Err(VmError::StackOverflow);
        }

        self.inner[idx] = Some(value);
        Ok(())
    }

    pub fn get_local(&self, frame_base: usize, local_index: usize) -> VmResult<&StackValue> {
        let idx = frame_base + local_index;
        if let Some(Some(value)) = self.inner.get(idx) {
            Ok(value)
        } else {
            Err(VmError::NullReference)
        }
    }
}

impl Debug for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stack")
            .field(
                "inner",
                &self
                    .inner
                    .iter()
                    .filter(|v| v.is_some())
                    .collect::<Vec<_>>(),
            )
            .field("top", &self.top)
            .field("capacity", &self.inner.len())
            .field("allocated", &self.total_alloc_size())
            .finish()
    }
}
