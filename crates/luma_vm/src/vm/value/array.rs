use std::alloc::Layout;

use crate::{value::StackValue, VmError, VmResult};

pub trait Array {
    fn capacity(&self) -> usize;
    fn get(&self, index: usize) -> VmResult<&StackValue>;
    fn get_mut(&mut self, index: usize) -> VmResult<&mut StackValue>;
    fn set(&mut self, index: usize, value: StackValue) -> VmResult<()>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FixedArray {
    storage: *mut StackValue,
    capacity: usize,
}

impl FixedArray {
    pub fn new(capacity: usize) -> Self {
        let layout = Layout::array::<StackValue>(capacity)
            .expect("failed to create stack layout");

        let storage = unsafe {
            let ptr = std::alloc::alloc(layout) as *mut StackValue;
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }

            ptr
        };

        Self {
            storage,
            capacity,
        }
    }
}

impl Array for FixedArray {
    fn get(&self, index: usize) -> VmResult<&StackValue> {
        if index >= self.capacity {
            return Err(VmError::IndexOutOfBounds(index));
        }

        unsafe {
            Ok(&*self.storage.add(index))
        }
    }

    fn get_mut(&mut self, index: usize) -> VmResult<&mut StackValue> {
        if index >= self.capacity {
            return Err(VmError::IndexOutOfBounds(index));
        }

        unsafe {
            Ok(&mut *self.storage.add(index))
        }
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn set(&mut self, index: usize, value: StackValue) -> VmResult<()> {
        if index >= self.capacity {
            return Err(VmError::IndexOutOfBounds(index));
        }

        unsafe {
            *self.storage.add(index) = value;
        }

        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynamicArray {
    elements: Vec<StackValue>,
}

impl DynamicArray {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<Vec<StackValue>> for DynamicArray {
    fn from(elements: Vec<StackValue>) -> Self {
        Self { elements }
    }
}

impl Array for DynamicArray {
    fn capacity(&self) -> usize {
        self.elements.len()
    }

    fn get(&self, index: usize) -> VmResult<&StackValue> {
        self.elements.get(index).ok_or(VmError::IndexOutOfBounds(index))
    }

    fn get_mut(&mut self, index: usize) -> VmResult<&mut StackValue> {
        self.elements.get_mut(index).ok_or(VmError::IndexOutOfBounds(index))
    }

    fn set(&mut self, index: usize, value: StackValue) -> VmResult<()> {
        if index >= self.elements.len() {
            self.elements.resize(index + 1, StackValue::Unit);
        }

        self.elements[index] = value;
        Ok(())
    }
}