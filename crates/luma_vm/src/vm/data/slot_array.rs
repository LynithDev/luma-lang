use std::fmt::Debug;

use luma_core::bytecode::IndexRef;

use crate::{VmError, VmResult};

pub struct SlotArray<T>
where T: Debug + Clone + PartialEq {
    inner: Box<[Option<T>]>,
    len: usize,
}

impl<T> SlotArray<T>
where T: Debug + Clone + PartialEq {
    pub fn new(len: usize) -> Self {
        Self {
            inner: vec![None; len].into_boxed_slice(),
            len,
        }
    }

    pub fn empty() -> Self {
        Self {
            inner: Box::new([]),
            len: 0,
        }
    }

    pub fn set(&mut self, index: usize, value: Option<T>) -> VmResult<IndexRef> {
        if index >= self.inner.len() {
            return Err(VmError::StackOverflow);
        }

        self.inner[index] = value;

        Ok(IndexRef::new(index))
    }

    pub fn try_get(&self, index: usize) -> VmResult<&T> {
        if let Some(Some(value)) = self.inner.get(index) {
            Ok(value)
        } else {
            Err(VmError::NullReference)
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index).and_then(|opt| opt.as_ref())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
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
        std::mem::size_of::<Self>() * self.capacity()
    }
}

impl<T> Debug for SlotArray<T>
where T: Debug + Clone + PartialEq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
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
            .field("len", &self.len)
            .field("capacity", &self.inner.len())
            .field("allocated", &self.total_alloc_size())
            .finish()
    }
}
