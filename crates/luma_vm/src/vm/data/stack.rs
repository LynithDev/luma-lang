use std::{alloc::Layout, fmt::Debug, ops::{Index, Range, RangeFrom}};

use crate::{VmError, VmResult};

pub struct Stack<T> {
    storage: *mut T,
    cap: usize,
    len: usize,
}

impl<T> Stack<T> {
    pub fn new(cap: usize) -> Self {
        let layout = Layout::array::<T>(cap)
            .expect("failed to create stack layout");

        let storage = unsafe {
            let ptr = std::alloc::alloc(layout) as *mut T;
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }

            ptr
        };

        Self {
            storage,
            cap,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> VmResult<usize> {
        let index = self.len;
        if index >= self.capacity() {
            return Err(VmError::StackOverflow);
        }

        unsafe {
            *self.storage.add(index) = value;
        }

        self.len += 1;
        Ok(index)
    }

    pub fn pop(&mut self) -> VmResult<T> {
        if self.len == 0 {
            return Err(VmError::StackUnderflow);
        }

        self.len -= 1;
        unsafe {
            Ok(std::ptr::read(self.storage.add(self.len)))
        }
    }

    pub fn pop_n(&mut self, n: usize) -> VmResult<()> {
        let new_len = self.len.saturating_sub(n);
        self.truncate_to(new_len)?;
        Ok(())
    }

    pub fn truncate_to(&mut self, new_len: usize) -> VmResult<()> {
        if new_len > self.len {
            return Err(VmError::StackOverflow);
        }

        unsafe {
            for i in new_len..self.len {
                std::ptr::drop_in_place(self.storage.add(i));
            }
        }
        self.len = new_len;

        Ok(())
    }

    pub fn peek(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                let index = self.len - 1;
                Some(&*self.storage.add(index))
            }
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                let index = self.len - 1;
                Some(&mut *self.storage.add(index))
            }
        }
    }

    pub fn try_peek(&self) -> VmResult<&T> {
        self.peek().ok_or(VmError::StackUnderflow)
    }

    pub fn try_peek_mut(&mut self) -> VmResult<&mut T> {
        self.peek_mut().ok_or(VmError::StackUnderflow)
    }

    #[must_use]
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.cap
    }

    #[must_use]
    #[inline(always)]
    pub fn total_alloc_size(&self) -> usize {
        std::mem::size_of::<Stack<T>>() * self.capacity()
    }

    pub fn get(&self, index: usize) -> VmResult<&T> {
        if index >= self.capacity() {
            return Err(VmError::IndexOutOfBounds(index));
        }

        unsafe {
            Ok(&*self.storage.add(index))
        }
    }

    pub fn get_mut(&mut self, index: usize) -> VmResult<&mut T> {
        if index >= self.capacity() {
            return Err(VmError::IndexOutOfBounds(index));
        }

        unsafe {
            Ok(&mut *self.storage.add(index))
        }
    }

    pub fn set(&mut self, index: usize, value: T) -> VmResult<()> {
        if index >= self.capacity() {
            return Err(VmError::IndexOutOfBounds(index));
        }

        unsafe {
            std::ptr::write(self.storage.add(index), value);
        }

        Ok(())
    }
}

impl<T> Index<usize> for Stack<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .as_ref()
            .expect("attempted to index null stack value")
    }
}

impl<T> Index<Range<usize>> for Stack<T> {
    type Output = [T];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        unsafe {
            std::slice::from_raw_parts(self.storage.add(range.start), range.end - range.start)
        }
    }
}

impl<T> Index<RangeFrom<usize>> for Stack<T> {
    type Output = [T];

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self[range.start..self.len]
    }
}

impl<T: Debug> Debug for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Stack<{}>", std::any::type_name::<T>()))
            .field(
                "inner",
                &self[0..self.len()]
                    .iter()
                    .enumerate()
                    .collect::<Vec<_>>(),
            )
            .field("len", &self.len())
            .field("capacity", &self.capacity())
            .field("allocated", &self.total_alloc_size())
            .finish()
    }
}