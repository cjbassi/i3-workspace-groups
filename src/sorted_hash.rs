use std::cmp::Ordering;
use std::collections::HashMap;

pub struct SortedHasher<T: Ord> {
    map: HashMap<usize, T>,
    max_hash: usize,
}

impl<T: Ord> SortedHasher<T> {
    pub fn new(size: usize) -> SortedHasher<T> {
        SortedHasher {
            max_hash: size,
            map: HashMap::new(),
        }
    }

    pub fn hash(&mut self, val: T) -> usize {
        let mut current_hash = self.max_hash / 2;
        loop {
            match self.map.get(&current_hash) {
                Some(x) => match val.cmp(x) {
                    Ordering::Equal => return current_hash,
                    Ordering::Less => current_hash /= 2,
                    Ordering::Greater => current_hash += current_hash / 2,
                },
                None => {
                    self.map.insert(current_hash, val);
                    return current_hash;
                }
            }
        }
    }

    pub fn set(&mut self, key: usize, val: T) {
        self.map.insert(key, val);
    }
}
