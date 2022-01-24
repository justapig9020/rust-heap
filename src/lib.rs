mod binary;
use std::cmp::Ord;
pub type HeapResult = Result<(), ()>;
pub trait Heap<T: Ord> {
    fn push(&mut self, val: T) -> HeapResult;
    fn pop(&mut self) -> Option<T>;
}