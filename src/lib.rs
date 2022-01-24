mod binary;
use std::cmp::Ord;
pub type HeapResult = Result<(), ()>;
pub trait Heap<T: Ord> {
    fn push(&mut self, val: T) -> HeapResult;
    fn pop(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
}

#[cfg(test)]
mod heap {
    use super::*;

    fn is_empty_tester(uut: &dyn Heap<i32>) {
        assert!(uut.is_empty());
    }
    fn isnt_empty_tester(uut: &mut dyn Heap<i32>) {
        uut.push(1).expect("Push failed");
        assert!(!uut.is_empty());
    }
    fn push_and_pop_min_tester(uut: &mut dyn Heap<i32>) {
        let mut values = [5, 3, 2, 4, 1];
        for v in values.iter() {
            uut.push(*v).expect("Push failed");
        }
        values.sort();
        for v in values.iter() {
            let poped = uut.pop().expect("Out of value");
            assert_eq!(*v, poped);
        }
    }
    fn push_and_pop_max_tester(uut: &mut dyn Heap<i32>) {
        let mut values = [5, 3, 2, 4, 1];
        for v in values.iter() {
            uut.push(*v).expect("Push failed");
        }
        values.sort();
        values.reverse();
        for v in values.iter() {
            let poped = uut.pop().expect("Out of value");
            assert_eq!(*v, poped);
        }
    }
    #[cfg(test)]
    mod binary_heap {
        use crate::binary::BinaryHeap;
        use super::*;
        #[test]
        fn is_empty_max() {
            let uut = BinaryHeap::new_max();
            is_empty_tester(&uut);
        }
        #[test]
        fn is_empty_min() {
            let uut = BinaryHeap::new_min();
            is_empty_tester(&uut);
        }
        #[test]
        fn isnt_empty_max() {
            let mut uut = BinaryHeap::new_max();
            isnt_empty_tester(&mut uut);
        }
        #[test]
        fn isnt_empty_min() {
            let mut uut = BinaryHeap::new_min();
            isnt_empty_tester(&mut uut);
        }
        #[test]
        fn push_and_pop_min() {
            let mut uut = BinaryHeap::new_min();
            push_and_pop_min_tester(&mut uut);
        }
        #[test]
        fn push_and_pop_max() {
            let mut uut = BinaryHeap::new_max();
            push_and_pop_max_tester(&mut uut);
        }
    }
}