use crate::{Heap, HeapResult, ModifiableHeap};
use std::cell::RefCell;
use std::cmp::{Eq, Ord};
use std::collections::{HashMap, LinkedList};
use std::hash::Hash;
use std::marker::Copy;
use std::rc::Rc;

type BinomialTreeRef<K, V> = Rc<RefCell<BinomialTree<K, V>>>;

struct BinomialTree<K: Ord, V> {
    key: K,
    val: V,
    subtrees: LinkedList<BinomialTreeRef<K, V>>,
    parent: Option<BinomialTreeRef<K, V>>,
}

impl<K: Ord, V> BinomialTree<K, V> {
    fn new(key: K, val: V) -> BinomialTree<K, V> {
        Self {
            key,
            val,
            subtrees: LinkedList::new(),
            parent: None,
        }
    }
    fn degree(&self) -> usize {
        self.subtrees.len()
    }
    fn leak_subtrees(&mut self) -> LinkedList<BinomialTreeRef<K, V>> {
        for root in self.subtrees.iter_mut() {
            root.borrow_mut().parent = None;
        }
        self.subtrees.split_off(0)
    }
    fn into_key_pair(self) -> (K, V) {
        (self.key, self.val)
    }
    fn get_parent(&self) -> Option<BinomialTreeRef<K, V>> {
        self.parent.clone()
    }
}

fn union<K: Ord, V>(
    parent: &BinomialTreeRef<K, V>,
    child: &BinomialTreeRef<K, V>,
) -> Result<(), ()> {
    if parent.borrow().degree() != child.borrow().degree() {
        return Err(());
    }
    child.borrow_mut().parent = Some(parent.clone());
    parent.borrow_mut().subtrees.push_back(child.clone());
    Ok(())
}

pub struct BinomialHeap<K, V>
where
    K: Ord + Hash,
{
    total: usize,
    top: Option<BinomialTreeRef<K, V>>,
    hash: HashMap<K, Vec<BinomialTreeRef<K, V>>>,
    trees: LinkedList<BinomialTreeRef<K, V>>,
    /// Return true is first argument is better than second.
    /// Return false otherwise.
    policy: fn(&K, &K) -> bool,
}

impl<K, V> BinomialHeap<K, V>
where
    K: Ord + Hash + Copy,
    V: Eq,
{
    pub fn new_min() -> Self {
        let policy = |a: &K, b: &K| -> bool { *a < *b };
        BinomialHeap::new_with_policy(policy)
    }
    pub fn new_max() -> Self {
        let policy = |a: &K, b: &K| -> bool { *a > *b };
        BinomialHeap::new_with_policy(policy)
    }
    fn new_with_policy(policy: fn(&K, &K) -> bool) -> Self {
        Self {
            total: 0,
            top: None,
            hash: HashMap::new(),
            trees: LinkedList::new(),
            policy,
        }
    }
    fn is_heigher(&self, a: &BinomialTreeRef<K, V>, b: &BinomialTreeRef<K, V>) -> bool {
        (self.policy)(&a.borrow().key, &b.borrow().key)
    }
    fn compare<'a>(
        &self,
        a: &'a BinomialTreeRef<K, V>,
        b: &'a BinomialTreeRef<K, V>,
    ) -> (&'a BinomialTreeRef<K, V>, &'a BinomialTreeRef<K, V>) {
        if self.is_heigher(a, b) {
            (a, b)
        } else {
            (b, a)
        }
    }
    fn heigher(
        &self,
        a: &BinomialTreeRef<K, V>,
        b: &BinomialTreeRef<K, V>,
    ) -> BinomialTreeRef<K, V> {
        self.compare(a, b).0.clone()
    }
    fn remove_top_tree(&mut self) -> Option<BinomialTreeRef<K, V>> {
        let top = self.top.take()?;
        let top_key = &top.borrow().key;
        let mut cursor = self.trees.cursor_front_mut();
        while let Some(root) = cursor.current() {
            if root.borrow().key == *top_key {
                return cursor.remove_current();
            }
            cursor.move_next();
        }
        None
    }
    fn heapify(&mut self) {
        let types = log2(self.total) + 1;
        let mut group: Vec<Vec<BinomialTreeRef<K, V>>> = (0..types).map(|_| Vec::new()).collect();
        for root in self.trees.iter() {
            let degree = root.borrow().degree();
            group[degree].push(root.clone());
        }
        self.trees.clear();
        for i in 0..(types - 1) {
            while group[i].len() > 1 {
                let a = group[i].pop().unwrap();
                let b = group[i].pop().unwrap();
                let (parent, child) = self.compare(&a, &b);
                union(parent, child).expect("Union failed");
                group[i + 1].push(parent.clone());
            }
            if let Some(root) = group[i].pop() {
                self.top = self
                    .top
                    .as_ref()
                    .and_then(|old| Some(self.heigher(&old, &root)))
                    .or(Some(root.clone()));
                self.trees.push_back(root);
            }
        }
        if let Some(root) = group[types - 1].pop() {
            self.top = self
                .top
                .as_ref()
                .and_then(|old| Some(self.heigher(&old, &root)))
                .or(Some(root.clone()));
            self.trees.push_back(root);
        }
    }
    fn find_pair(&self, key: &K, val: &V) -> Option<BinomialTreeRef<K, V>> {
        let matches = self.hash.get(key)?;
        let got = matches.iter().find(|root| root.borrow().val == *val)?;
        Some(got.clone())
    }
    fn heapify_button_up(&mut self, modified: BinomialTreeRef<K, V>) {
        use std::mem;
        let mut curr = modified;
        loop {
            let parent = if let Some(p) = curr.borrow().get_parent() {
                p
            } else {
                self.update_top(&curr);
                break;
            };
            if self.is_heigher(&parent, &curr) {
                break;
            }
            mem::swap(&mut parent.borrow_mut().key, &mut curr.borrow_mut().key);
            mem::swap(&mut parent.borrow_mut().val, &mut curr.borrow_mut().val);
            curr = parent;
        }
    }
    fn update_top(&mut self, challenger: &BinomialTreeRef<K, V>) {
        self.top = self
            .top
            .take()
            .and_then(|top| Some(self.heigher(&top, challenger)))
            .or(Some(challenger.clone()));
    }
    fn hash_insert(&mut self, key: &K, root: &BinomialTreeRef<K, V>) {
        if let Some(vec) = self.hash.get_mut(&key) {
            vec.push(root.clone());
        } else {
            self.hash.insert(*key, vec![root.clone()]);
        }
    }
    fn modify_hash_mapping(&mut self, root: &BinomialTreeRef<K, V>, new_key: &K) -> Result<(), ()> {
        let old_key = &root.borrow().key;
        let vec = self.hash.get_mut(old_key).ok_or(())?;
        vec.retain(|r| r.borrow().key != *old_key);
        self.hash_insert(new_key, root);
        Ok(())
    }
}

impl<K, V> Heap<K, V> for BinomialHeap<K, V>
where
    K: Ord + Hash + Copy,
    V: Eq,
{
    fn push(&mut self, key: K, val: V) -> HeapResult {
        let new_tree = Rc::new(RefCell::new(BinomialTree::new(key, val)));
        self.hash_insert(&key, &new_tree);
        self.update_top(&new_tree);
        self.trees.push_back(new_tree);
        self.total += 1;
        Ok(())
    }
    fn pop(&mut self) -> Option<(K, V)> {
        let top = self.remove_top_tree()?;
        let mut subtrees = top.borrow_mut().leak_subtrees();
        self.trees.append(&mut subtrees);
        self.heapify();
        let mut vec = self.hash.remove(&top.borrow().key)?;
        vec.retain(|b| b.borrow().val != top.borrow().val);
        if !vec.is_empty() {
            self.hash.insert(top.borrow().key, vec);
        }
        let cell = Rc::try_unwrap(top).ok()?;
        let tree = cell.into_inner();
        self.total -= 1;
        Some(tree.into_key_pair())
    }
    fn len(&self) -> usize {
        self.total
    }
    fn is_empty(&self) -> bool {
        self.trees.is_empty()
    }
    fn peek(&self) -> Option<(&K, &V)> {
        self.top.as_ref().map(|top| {
            let top_ptr = top.as_ptr();
            // SAFETY: returned pair is behind immutable reference of self.
            //         Since key and value will only be modified in function with mutable reference of self,
            //         the returned reference are safed.
            let top_ref = unsafe { top_ptr.as_ref() }.unwrap();
            (&top_ref.key, &top_ref.val)
        })
    }
}

impl<K, V> ModifiableHeap<K, V> for BinomialHeap<K, V>
where
    K: Ord + Hash + Copy,
    V: Eq,
{
    fn modify_key(&mut self, old_key: &K, val: &V, new_key: K) -> HeapResult {
        let modify = self.find_pair(old_key, val).ok_or(())?;
        self.modify_hash_mapping(&modify, &new_key)?;
        modify.borrow_mut().key = new_key;
        self.heapify_button_up(modify);
        Ok(())
    }
}

#[test]
fn log2_test() {
    let v = 1025;
    assert_eq!(10, log2(v));
}

fn log2(mut n: usize) -> usize {
    let mut cnt = 0;
    n >>= 1;
    while n > 0 {
        cnt += 1;
        n >>= 1;
    }
    cnt
}
