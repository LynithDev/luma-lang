#[derive(Debug)]
pub struct Arena<T> {
    inner: Vec<Box<T>>,
}

impl<T> Arena<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn alloc(&mut self, value: T) -> *const T {
        let b = Box::new(value);
        let ptr = &*b as *const T;
        self.inner.push(b);
        ptr
    }

    pub fn get_unchecked(&self, index: usize) -> &T {
        &self.inner[index]
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index).map(|b| &**b)
    }

    pub fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
        &mut self.inner[index]
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index).map(|b| &mut **b)
    }

    pub fn get_ptr(&self, index: usize) -> *const T {
        &*self.inner[index] as *const T
    }

    pub fn get_mut_ptr(&mut self, index: usize) -> *mut T {
        &mut *self.inner[index] as *mut T
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn total_alloc_size(&self) -> usize {
        std::mem::size_of::<Box<T>>() * self.inner.len()
    }
}

impl<T> From<Vec<T>> for Arena<T> {
    fn from(vec: Vec<T>) -> Self {
        let inner = vec.into_iter().map(Box::new).collect();
        Self { inner }
    }
}
