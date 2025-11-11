use crate::{runtime::CallFrame, VmError, VmResult};

#[derive(Debug)]
pub struct Frames {
    inner: Vec<CallFrame>,
}

impl Frames {
    pub fn new(len: usize) -> Self {
        Self {
            inner: Vec::with_capacity(len),
        }
    }

    pub fn push(&mut self, frame: CallFrame) -> VmResult<()> {
        let index = self.inner.len();
        if index >= self.inner.capacity() {
            return Err(VmError::MaxFrameCountExceeded);
        }
        
        self.inner.push(frame);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<CallFrame> {
        self.inner.pop()
    }

    pub fn last_mut(&mut self) -> VmResult<&mut CallFrame> {
        self.inner.last_mut().ok_or(VmError::NoActiveCallFrame)
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
}
