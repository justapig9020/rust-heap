mod binary;
mod binomial;

use std::cmp::Ord;
use std::hash::Hash;
pub type HeapResult = Result<(), ()>;
pub trait Heap<K: Ord, V> {
    fn push(&mut self, key: K, val: V) -> HeapResult;
    fn pop(&mut self) -> Option<(K, V)>;
    fn is_empty(&self) -> bool;
}

#[cfg(test)]
mod heap {
    use super::*;

    fn is_empty_tester(uut: &dyn Heap<i32, ()>) {
        assert!(uut.is_empty());
    }
    fn isnt_empty_tester(uut: &mut dyn Heap<i32, ()>) {
        uut.push(1, ()).expect("Push failed");
        assert!(!uut.is_empty());
    }
    fn push_and_pop_min_tester(uut: &mut dyn Heap<i32, String>) {
        let mut values = [5, 3, 2, 4, 1];
        for v in values.iter() {
            uut.push(*v, v.to_string()).expect("Push failed");
        }
        values.sort();
        for v in values.iter() {
            let (key, val) = uut.pop().expect("Out of value");
            assert_eq!(*v, key);
            assert_eq!(v.to_string(), val);
        }
    }
    fn push_and_pop_max_tester(uut: &mut dyn Heap<i32, String>) {
        let mut values = [5, 3, 2, 4, 1];
        for v in values.iter() {
            uut.push(*v, v.to_string()).expect("Push failed");
        }
        values.sort();
        values.reverse();
        for v in values.iter() {
            let (key, val) = uut.pop().expect("Out of value");
            assert_eq!(*v, key);
            assert_eq!(v.to_string(), val);
        }
    }
    #[cfg(test)]
    mod binary_heap {
        use super::*;
        use crate::binary::BinaryHeap;
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

pub trait ModifiableHeap<K, V>: Heap<K, V>
where
    K: Ord + Hash,
{
    fn modify(&mut self, key: K, new_val: V) -> HeapResult;
}
