use std::{fmt::Debug, ops::{Index, Range, RangeFrom}};

use luma_core::bytecode::IndexRef;

use crate::{VmError, VmResult, value::StackValue};

pub struct Stack {
    inner: *mut StackValue,
    cap: usize,
    count: usize,
}

impl Stack {
    pub fn new(len: usize) -> Self {
        Self {
            inner: unsafe {
                let layout = std::alloc::Layout::array::<StackValue>(len)
                    .expect("failed to create stack layout");
                let ptr = std::alloc::alloc_zeroed(layout) as *mut StackValue;
                if ptr.is_null() {
                    std::alloc::handle_alloc_error(layout);
                }
                ptr
            },
            cap: len,
            count: 0,
        }
    }

    pub fn push(&mut self, value: StackValue) -> VmResult<IndexRef> {
        let index = self.count;
        if index >= self.capacity() {
            return Err(VmError::StackOverflow);
        }

        unsafe {
            *self.inner.add(index) = value;
        }

        self.count += 1;
        Ok(IndexRef::new(index))
    }

    pub fn pop(&mut self) -> VmResult<StackValue> {
        if self.count == 0 {
            return Err(VmError::StackUnderflow);
        }

        self.count -= 1;
        unsafe {
            Ok(std::ptr::read(self.inner.add(self.count)))
        }
    }

    pub fn pop_n(&mut self, n: usize) -> VmResult<()> {
        let new_len = self.count.saturating_sub(n);
        self.truncate_to(new_len)?;
        Ok(())
    }

    pub fn truncate_to(&mut self, new_len: usize) -> VmResult<()> {
        if new_len > self.count {
            return Err(VmError::StackOverflow);
        }

        // while self.count > new_len {
        //     self.count -= 1;
        //     self.inner[self.count] = None;
        // }

        self.count = new_len;
        unsafe {
            for i in new_len..self.cap {
                std::ptr::drop_in_place(self.inner.add(i));
            }
        }

        Ok(())
    }

    pub fn peek(&self) -> Option<&StackValue> {
        if self.count == 0 {
            None
        } else {
            unsafe {
                let index = self.count - 1;
                Some(&*self.inner.add(index))
            }
        }
    }

    pub fn try_peek(&self) -> VmResult<&StackValue> {
        self.peek().ok_or(VmError::StackUnderflow)
    }

    #[must_use]
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.count
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
        std::mem::size_of::<Stack>() * self.capacity()
    }

    pub fn get(&self, index: usize) -> VmResult<&StackValue> {
        // if let Some(Some(value)) = self.inner.get(index) {
        //     Ok(value)
        // } else {
        //     Err(VmError::NullReference)
        // }
        if index >= self.capacity() {
            return Err(VmError::IndexOutOfBounds(index));
        }

        unsafe {
            Ok(&*self.inner.add(index))
        }
    }

    pub fn get_mut(&mut self, index: usize) -> VmResult<&mut StackValue> {
        // if let Some(Some(value)) = self.inner.get_mut(index) {
        //     Ok(value)
        // } else {
        //     Err(VmError::NullReference)
        // }
        if index >= self.capacity() {
            return Err(VmError::IndexOutOfBounds(index));
        }

        unsafe {
            Ok(&mut *self.inner.add(index))
        }
    }

    pub fn set(&mut self, index: usize, value: Option<StackValue>) -> VmResult<()> {
        if index >= self.capacity() {
            return Err(VmError::IndexOutOfBounds(index));
        }

        // self.inner[index] = value;
        unsafe {
            std::ptr::write(self.inner.add(index), value.unwrap());
        }

        Ok(())
    }
}

impl Index<usize> for Stack {
    type Output = StackValue;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .as_ref()
            .expect("attempted to index null stack value")
    }
}

impl Index<Range<usize>> for Stack {
    type Output = [StackValue];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        unsafe {
            std::slice::from_raw_parts(self.inner.add(range.start), range.end - range.start)
        }
    }
}

impl Index<RangeFrom<usize>> for Stack {
    type Output = [StackValue];

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self[range.start..self.count]
    }
}

impl Debug for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stack")
            .field(
                "inner",
                &self[0..]
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

// use std::{fmt::Debug, ops::{Index, Range, RangeFrom}};

// use luma_core::bytecode::IndexRef;

// use crate::{VmError, VmResult, value::StackValue};

// pub struct Stack {
//     inner: Box<[Option<StackValue>]>,
//     pub count: usize,
// }

// impl Stack {
//     pub fn new(len: usize) -> Self {
//         Self {
//             inner: vec![None; len].into_boxed_slice(),
//             count: 0,
//         }
//     }

//     pub fn push(&mut self, value: StackValue) -> VmResult<IndexRef> {
//         let index = self.count;
//         if index >= self.capacity() {
//             return Err(VmError::StackOverflow);
//         }

//         self.inner[self.count] = Some(value);
//         self.count += 1;

//         Ok(IndexRef::new(index))
//     }

//     pub fn pop(&mut self) -> VmResult<StackValue> {
//         if self.count == 0 {
//             return Err(VmError::StackUnderflow);
//         }

//         self.count -= 1;
//         self.inner[self.count].take().ok_or(VmError::NullReference)
//     }

//     pub fn pop_n(&mut self, n: usize) -> VmResult<()> {
//         let new_len = self.count.saturating_sub(n);
//         self.truncate_to(new_len)?;
//         Ok(())
//     }

//     pub fn truncate_to(&mut self, new_len: usize) -> VmResult<()> {
//         if new_len > self.count {
//             return Err(VmError::StackOverflow);
//         }

//         while self.count > new_len {
//             self.count -= 1;
//             self.inner[self.count] = None;
//         }

//         Ok(())
//     }

//     pub fn peek(&self) -> Option<&StackValue> {
//         if self.count == 0 {
//             None
//         } else {
//             self.inner[self.count - 1].as_ref()
//         }
//     }

//     pub fn try_peek(&self) -> VmResult<&StackValue> {
//         self.peek().ok_or(VmError::StackUnderflow)
//     }

//     #[must_use]
//     pub fn len(&self) -> usize {
//         self.count
//     }

//     #[must_use]
//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }

//     #[must_use]
//     pub fn capacity(&self) -> usize {
//         self.inner.len()
//     }

//     #[must_use]
//     pub fn total_alloc_size(&self) -> usize {
//         std::mem::size_of::<Stack>() * self.capacity()
//     }

//     pub fn get(&self, index: usize) -> VmResult<&StackValue> {
//         if let Some(Some(value)) = self.inner.get(index) {
//             Ok(value)
//         } else {
//             Err(VmError::NullReference)
//         }
//     }

//     pub fn get_mut(&mut self, index: usize) -> VmResult<&mut StackValue> {
//         if let Some(Some(value)) = self.inner.get_mut(index) {
//             Ok(value)
//         } else {
//             Err(VmError::NullReference)
//         }
//     }

//     pub fn set(&mut self, index: usize, value: Option<StackValue>) -> VmResult<()> {
//         if index >= self.capacity() {
//             return Err(VmError::IndexOutOfBounds(index));
//         }

//         self.inner[index] = value;
//         Ok(())
//     }
// }

// impl Index<usize> for Stack {
//     type Output = StackValue;

//     fn index(&self, index: usize) -> &Self::Output {
//         self.inner[index]
//             .as_ref()
//             .expect("attempted to index null stack value")
//     }
// }

// impl Index<Range<usize>> for Stack {
//     type Output = [Option<StackValue>];

//     fn index(&self, range: Range<usize>) -> &Self::Output {
//         &self.inner[range]
//     }
// }

// impl Index<RangeFrom<usize>> for Stack {
//     type Output = [Option<StackValue>];

//     fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
//         &self.inner[range.start..self.count]
//     }
// }

// impl Debug for Stack {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Stack")
//             .field(
//                 "inner",
//                 &self
//                     .inner
//                     .iter()
//                     .enumerate()
//                     .filter_map(|(index, item)| {
//                         Some((index, item.as_ref()?))
//                     })
//                     .collect::<Vec<_>>(),
//             )
//             .field("len", &self.len())
//             .field("capacity", &self.inner.len())
//             .field("allocated", &self.total_alloc_size())
//             .finish()
//     }
// }
