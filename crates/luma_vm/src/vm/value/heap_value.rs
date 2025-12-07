use std::rc::Rc;

use crate::value::{Closure, DynamicArray};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HeapValue {
    String(Rc<String>),
    DynamicArray(DynamicArray),
    Closure(*mut Closure),
}