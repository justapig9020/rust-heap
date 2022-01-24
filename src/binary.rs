use crate::{ Heap, HeapResult };
use std::cmp::Ord;
pub struct BinaryHeap<T: Ord> {
    heap: Vec<T>,
    policy: fn(&T, &T) -> bool,
}

impl <T: Ord> Heap<T> for BinaryHeap<T> {
    fn push(&mut self, val: T) -> HeapResult {
        self.heap.push(val);
        self.heapify_button_up();
        Ok(())
    }
    fn pop(&mut self) -> Option<T> {
        let n = self.heap.len();
        if n == 0 {
            None
        } else if n == 1 {
            self.heap.pop()
        } else {
            self.heap.swap(0, n - 1);
            let ret = self.heap.pop();
            self.heapify_top_button();
            ret
        }
    }
    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

impl <T: Ord> BinaryHeap<T> {
    pub fn new_min() -> Self {
        let policy = |a: &T, b: &T| -> bool {
            *a < *b
        };
        Self {
            heap: vec![],
            policy,
        }
    }
    pub fn new_max() -> Self {
        let policy = |a: &T, b: &T| -> bool {
            *a > *b
        };
        Self {
            heap: vec![],
            policy,
        }
    }
    fn heapify_button_up(&mut self) {
        let mut curr = self.heap.len() - 1;
        while curr > 0 {
            let parent = parent_of(curr);
            if !self.should_heigher(parent, curr) {
                self.heap.swap(parent, curr);
                curr = parent;
            } else {
                break;
            }
        }
    }
    fn heapify_top_button(&mut self) {
        let mut curr = 0;
        let last = self.heap.len() - 1;
        if last == 0 {
            return;
        }
        let last_parent = parent_of(last);
        while curr <= last_parent {
            let lchild = lchild_of(curr);
            let rchild = rchild_of(curr);
            let to_check = if rchild > last {
                lchild
            } else {
                if self.should_heigher(lchild, rchild) {
                    lchild
                } else {
                    rchild
                }
            };
            if !self.should_heigher(curr, to_check) {
                self.heap.swap(curr, to_check);
                curr = to_check;
            } else {
                break;
            }
        }
    }
    /// Check whether "i" should be heigher than "j"
    fn should_heigher(&self, i: usize, j: usize) -> bool {
        let heap = &self.heap;
        (self.policy)(&heap[i], &heap[j])
    }
}

fn parent_of(idx: usize) -> usize {
    (idx - 1) / 2
}
fn lchild_of(idx: usize) -> usize {
    idx * 2 + 1
}
fn rchild_of(idx: usize) -> usize {
    idx * 2 + 2
}