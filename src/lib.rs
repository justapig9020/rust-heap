#![feature(linked_list_cursors)]
pub mod binary;
pub mod binomial;

use std::cmp::Ord;
use std::hash::Hash;
pub type HeapResult = Result<(), ()>;
pub trait Heap<K: Ord, V> {
    fn push(&mut self, key: K, val: V) -> HeapResult;
    fn pop(&mut self) -> Option<(K, V)>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn peek(&self) -> Option<(&K, &V)>;
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
    fn pop_empty_tester(uut: &mut dyn Heap<i32, ()>) {
        let got = uut.pop();
        assert!(got.is_none());
    }
    fn peek_some_tester(uut: &mut dyn Heap<i32, String>) {
        let pairs: Vec<(i32, String)> = (0..10).map(|i| (i, i.to_string())).collect();
        for (k, v) in pairs.iter() {
            uut.push(*k, v.clone());
        }
        let (k, v) = &pairs[9];
        let (key, val) = uut.peek().expect("Expect Some, got None");
        assert_eq!(*k, *key);
        assert_eq!(*v, *val);
    }
    fn peek_none_tester(uut: &dyn Heap<i32, String>) {
        let got = uut.peek();
        assert!(got.is_none());
    }
    fn len_after_push_tester(uut: &mut dyn Heap<usize, ()>) {
        assert_eq!(uut.len(), 0);
        let len: usize = 10;
        for i in 0..len {
            uut.push(i, ());
        }
        assert_eq!(uut.len(), len);
    }
    fn len_after_pop_tester(uut: &mut dyn Heap<usize, ()>) {
        assert_eq!(uut.len(), 0);
        let len: usize = 10;
        for i in 0..len {
            uut.push(i, ());
        }
        for i in 1..=len {
            uut.pop();
            assert_eq!(uut.len(), len - i);
        }
    }
    fn len_empty_tester(uut: &dyn Heap<usize, ()>) {
        assert_eq!(uut.len(), 0);
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
        #[test]
        fn pop_empty() {
            let mut uut = BinaryHeap::new_max();
            pop_empty_tester(&mut uut);
        }
        #[test]
        fn peek_some() {
            let mut uut = BinaryHeap::new_max();
            peek_some_tester(&mut uut);
        }
        #[test]
        fn peek_empty() {
            let mut uut = BinaryHeap::new_max();
            peek_none_tester(&mut uut);
        }
        #[test]
        fn len_after_push() {
            let mut uut = BinaryHeap::new_max();
            len_after_push_tester(&mut uut);
        }
        #[test]
        fn len_after_pop() {
            let mut uut = BinaryHeap::new_max();
            len_after_pop_tester(&mut uut);
        }
        #[test]
        fn len_empty() {
            let uut = BinaryHeap::new_max();
            len_empty_tester(&uut);
        }
    }

    #[cfg(test)]
    mod binomial_heap {
        use super::*;
        use crate::binomial::BinomialHeap;
        #[test]
        fn is_empty_max() {
            let uut = BinomialHeap::new_max();
            is_empty_tester(&uut);
        }
        #[test]
        fn is_empty_min() {
            let uut = BinomialHeap::new_min();
            is_empty_tester(&uut);
        }
        #[test]
        fn isnt_empty_max() {
            let mut uut = BinomialHeap::new_max();
            isnt_empty_tester(&mut uut);
        }
        #[test]
        fn isnt_empty_min() {
            let mut uut = BinomialHeap::new_min();
            isnt_empty_tester(&mut uut);
        }
        #[test]
        fn push_and_pop_min() {
            let mut uut = BinomialHeap::new_min();
            push_and_pop_min_tester(&mut uut);
        }
        #[test]
        fn push_and_pop_max() {
            let mut uut = BinomialHeap::new_max();
            push_and_pop_max_tester(&mut uut);
        }
        #[test]
        fn pop_empty() {
            let mut uut = BinomialHeap::new_max();
            pop_empty_tester(&mut uut);
        }
        #[test]
        fn repeat_key() {
            let mut uut = BinomialHeap::new_max();
            let pair: Vec<(usize, String)> = [(1, "key1"), (1, "key2"), (2, "key3"), (2, "key4")]
                .iter()
                .map(|(i, s)| (*i, (*s).to_string()))
                .collect();
            for (k, v) in pair.iter() {
                uut.push(*k, v.clone());
            }
            let mut got = vec![];
            while !uut.is_empty() {
                got.push(uut.pop().unwrap());
            }
            assert!(pair.iter().all(|p| got.contains(p)));
        }
        #[test]
        fn peek_some() {
            let mut uut = BinomialHeap::new_max();
            peek_some_tester(&mut uut);
        }
        #[test]
        fn peek_empty() {
            let mut uut = BinomialHeap::new_max();
            peek_none_tester(&mut uut);
        }
        #[test]
        fn len_after_push() {
            let mut uut = BinomialHeap::new_max();
            len_after_push_tester(&mut uut);
        }
        #[test]
        fn len_after_pop() {
            let mut uut = BinomialHeap::new_max();
            len_after_pop_tester(&mut uut);
        }
        #[test]
        fn len_empty() {
            let uut = BinomialHeap::new_max();
            len_empty_tester(&uut);
        }
    }
}

#[cfg(test)]
mod modifiable_heap {
    use super::*;
    // Minheap
    fn modify_key_tester(uut: &mut dyn ModifiableHeap<i32, String>) {
        let mut list: Vec<(i32, String)> = (0..10).map(|i| (i, i.to_string())).collect();
        for (k, v) in list.iter() {
            uut.push(*k, v.clone());
        }
        let mut cnt = 10;
        for (k, v) in list.iter_mut() {
            uut.modify_key(k, v, cnt);
            *k = cnt;
            cnt -= 1;
        }
        list.sort_by_key(|(k, _)| *k);
        for (k, v) in list {
            let (key, val) = uut.pop().unwrap();
            assert_eq!(k, key);
            assert_eq!(v, val);
        }
    }
    #[cfg(test)]
    mod binomial_heap {
        use super::*;
        use crate::binomial::BinomialHeap;
        #[test]
        fn modify_key() {
            let mut uut = BinomialHeap::new_min();
            modify_key_tester(&mut uut);
        }
    }
}
pub trait ModifiableHeap<K, V>: Heap<K, V>
where
    K: Ord + Hash,
{
    fn modify_key(&mut self, old_key: &K, val: &V, new_key: K) -> HeapResult;
}
