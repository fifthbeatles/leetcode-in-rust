/**
 * [146. LRU Cache](https://leetcode.com/problems/lru-cache/description/)
 */
use std::collections::HashMap;

use crate::data_structures::doubly_linked_list::{DoublyList, NodeRef};

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, NodeRef<(i32, i32)>>,
    lru: DoublyList<(i32, i32)>, // (key, value)
}

#[allow(dead_code)]
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            map: HashMap::with_capacity(capacity as usize),
            lru: DoublyList::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            self.lru.move_to_head(node.clone());
            node.borrow().value.1
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key) {
            node.borrow_mut().value.1 = value;
            self.lru.move_to_head(node.clone());
        } else {
            if self.map.len() >= self.capacity {
                let (k, _) = self.lru.pop_back().unwrap();
                self.map.remove(&k);
            }
            self.lru.push_front((key, value));
            self.map.insert(key, self.lru.get_head().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 11);
        lru_cache.put(2, 12);
        assert_eq!(lru_cache.get(1), 11);
        lru_cache.put(3, 13);
        assert_eq!(lru_cache.get(2), -1);
        lru_cache.put(4, 14);
        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 13);
        assert_eq!(lru_cache.get(4), 14);
    }
}
