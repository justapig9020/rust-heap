use crate::{ Heap, HeapResult };
use std::cmp::Ord;
pub struct BinaryHeap<T: Ord> {
    heap: Vec<T>,
}

impl <T: Ord> Heap<T> for BinaryHeap<T> {
    fn push(&mut self, val: T) -> HeapResult {
        Ok(())
    }
    fn pop(&mut self) -> Option<T> {
        None
    }
}

impl <T: Ord> BinaryHeap<T> {
    pub fn new_min() -> Self {
        Self {
            heap: vec![]
        }
    }
    pub fn new_max() -> Self {
        Self {
            heap: vec![]
        }
    }
}