use std::fmt::Debug;

use luma_core::bytecode::IndexRef;

use crate::{VmError, VmResult, value::StackValue};

pub struct Stack {
    inner: Box<[Option<StackValue>]>,
    count: usize,
}

impl Stack {
    pub fn new(len: usize) -> Self {
        Self {
            inner: vec![None; len].into_boxed_slice(),
            count: 0,
        }
    }

    pub fn push(&mut self, value: StackValue) -> VmResult<IndexRef> {
        let index = self.count;
        if index >= self.inner.len() {
            return Err(VmError::StackOverflow);
        }

        self.inner[self.count] = Some(value);
        self.count += 1;

        Ok(IndexRef::new(index))
    }

    pub fn pop(&mut self) -> VmResult<StackValue> {
        if self.count == 0 {
            return Err(VmError::StackUnderflow);
        }

        self.count -= 1;
        self.inner[self.count].take().ok_or(VmError::NullReference)
    }

    pub fn peek(&self) -> Option<&StackValue> {
        if self.count == 0 {
            None
        } else {
            self.inner[self.count - 1].as_ref()
        }
    }

    pub fn try_peek(&self) -> VmResult<&StackValue> {
        self.peek().ok_or(VmError::StackUnderflow)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.count
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

    pub fn at(&self, index: usize) -> VmResult<&StackValue> {
        if let Some(Some(value)) = self.inner.get(index) {
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
                    .enumerate()
                    .filter_map(|(index, item)| {
                        Some((index, item.as_ref()?))
                    })
                    .collect::<Vec<_>>(),
            )
            .field("len", &self.len())
            .field("capacity", &self.inner.len())
            .field("allocated", &self.total_alloc_size())
            .finish()
    }
}
