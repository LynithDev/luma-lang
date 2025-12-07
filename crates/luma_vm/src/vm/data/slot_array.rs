use std::{fmt::Debug, hash::Hash};

use crate::{VmError, VmResult};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SlotArray<T>
where T: Debug + Clone + PartialEq + Eq + Hash {
    inner: Box<[Option<T>]>,
    len: usize,
}

impl<T> SlotArray<T>
where T: Debug + Clone + PartialEq + Eq + Hash {
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

    pub fn set(&mut self, index: usize, value: Option<T>) -> VmResult<usize> {
        if index >= self.inner.len() {
            return Err(VmError::StackOverflow);
        }

        self.inner[index] = value;

        Ok(index)
    }

    pub fn clear_range(&mut self, start: usize, end: usize) -> VmResult<()> {
        if end > self.inner.len() || start >= end {
            return Err(VmError::IndexOutOfBounds(end));
        }

        for index in start..end {
            self.inner[index] = None;
        }

        Ok(())
    }

    pub fn try_get(&self, index: usize) -> VmResult<&T> {
        if let Some(Some(value)) = self.inner.get(index) {
            Ok(value)
        } else {
            Err(VmError::NullReference)
        }
    }

    pub fn try_get_mut(&mut self, index: usize) -> VmResult<&mut T> {
        if let Some(Some(value)) = self.inner.get_mut(index) {
            Ok(value)
        } else {
            Err(VmError::NullReference)
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index).and_then(|opt| opt.as_ref())
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index).and_then(|opt| opt.as_mut())
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

impl<T> From<Vec<T>> for SlotArray<T>
where T: Debug + Clone + PartialEq + Eq + Hash {
    fn from(vec: Vec<T>) -> Self {
        let len = vec.len();
        let mut slot_array = SlotArray::new(len);

        for (index, value) in vec.into_iter().enumerate() {
            // Safe to unwrap since we just created the SlotArray with sufficient capacity
            slot_array.set(index, Some(value)).unwrap();
        }

        slot_array
    }
}

impl<T> Debug for SlotArray<T>
where T: Debug + Clone + PartialEq + Eq + Hash {
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
