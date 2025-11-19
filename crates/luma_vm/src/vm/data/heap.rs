use luma_core::bytecode::IndexRef;

use crate::{value::HeapValue, VmError, VmResult};

#[derive(Debug)]
pub struct Heap {
    inner: Vec<HeapValue>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
        }
    }

    pub fn push(&mut self, value: HeapValue) -> VmResult<IndexRef> {
        let index = self.inner.len();
        self.inner.push(value);
        Ok(IndexRef::new(index))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[must_use]
    pub fn get(&self, index: IndexRef) -> Option<&HeapValue> {
        self.inner.get(*index)
    }

    pub fn try_get(&mut self, index: IndexRef) -> VmResult<&HeapValue> {
        self.get(index).ok_or(VmError::NullReference)
    }

    pub fn set(&mut self, index: IndexRef, value: HeapValue) -> VmResult<()> {
        self.inner[*index] = value;
        Ok(())
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}
