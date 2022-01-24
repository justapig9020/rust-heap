use std::collections::{ HashMap, LinkedList };
use std::cmp::Ord;
use std::hash::Hash;
use crate::{ Heap, ModifiableHeap, HeapResult };

struct BinomialTree {

}

pub struct BinomialHeap {

}

impl <K: Ord, V> Heap<K, V> for BinomialHeap {
    fn push(&mut self, key: K, val: V) -> HeapResult {
        Ok(())
    }
    fn pop(&mut self) -> Option<(K, V)> {
        None
    }
    fn is_empty(&self) -> bool {
        false
    }
}

impl <K, V> ModifiableHeap<K, V> for BinomialHeap 
    where
        K: Ord + Hash
{
    fn modify(&mut self, key: K, new_val: V) -> HeapResult {
        Ok(())
    }
}