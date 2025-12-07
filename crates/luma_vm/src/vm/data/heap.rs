use std::fmt::Debug;

use crate::{value::HeapValue, VmError, VmResult};

pub struct Heap {
    inner: Vec<HeapValue>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
        }
    }

    pub fn push(&mut self, value: HeapValue) -> VmResult<usize> {
        let index = self.inner.len();
        self.inner.push(value);
        Ok(index)
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
    pub fn get(&self, index: usize) -> Option<&HeapValue> {
        self.inner.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut HeapValue> {
        self.inner.get_mut(index)
    }

    pub fn try_get(&self, index: usize) -> VmResult<&HeapValue> {
        self.get(index).ok_or(VmError::NullReference)
    }

    pub fn try_get_mut(&mut self, index: usize) -> VmResult<&mut HeapValue> {
        self.get_mut(index).ok_or(VmError::NullReference)
    }

    pub fn set(&mut self, index: usize, value: HeapValue) -> VmResult<()> {
        self.inner[index] = value;
        Ok(())
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for Heap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Heap")
            .field(
                "inner",
                &self
                    .inner
                    .iter()
                    .enumerate()
                    .collect::<Vec<_>>(),
            )
            .field("len", &self.len())
            .field("capacity", &self.inner.capacity())
            .finish()
    }
}
