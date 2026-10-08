//! A generic LRU cache. Safe-Rust design: a HashMap for lookup plus a
//! VecDeque of keys ordered from least to most recently used.
//! `touch` is O(n); the README challenge asks you to make it O(1).

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>, // front = least recently used
}

impl<K: Hash + Eq + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        LruCache { capacity, map: HashMap::with_capacity(capacity), order: VecDeque::new() }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn touch(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            let k = self.order.remove(pos).unwrap();
            self.order.push_back(k);
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.touch(key);
        }
        self.map.get(key)
    }

    /// Inserts a value; returns the evicted entry if the cache was full.
    pub fn put(&mut self, key: K, value: V) -> Option<(K, V)> {
        if self.map.insert(key.clone(), value).is_some() {
            self.touch(&key);
            return None;
        }
        self.order.push_back(key);
        if self.map.len() > self.capacity {
            let oldest = self.order.pop_front()?;
            let v = self.map.remove(&oldest)?;
            return Some((oldest, v));
        }
        None
    }

    /// Looks without changing recency.
    pub fn peek(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evicts_least_recently_used() {
        let mut c = LruCache::new(2);
        c.put("a", 1);
        c.put("b", 2);
        assert_eq!(c.put("c", 3), Some(("a", 1)));
        assert_eq!(c.peek(&"a"), None);
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn get_refreshes_recency() {
        let mut c = LruCache::new(2);
        c.put("a", 1);
        c.put("b", 2);
        assert_eq!(c.get(&"a"), Some(&1));
        assert_eq!(c.put("c", 3), Some(("b", 2)));
    }

    #[test]
    fn overwrite_does_not_evict() {
        let mut c = LruCache::new(2);
        c.put("a", 1);
        c.put("b", 2);
        assert_eq!(c.put("a", 10), None);
        assert_eq!(c.get(&"a"), Some(&10));
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn works_with_owned_keys() {
        let mut c: LruCache<String, Vec<u8>> = LruCache::new(1);
        c.put("x".to_string(), vec![1]);
        assert!(c.put("y".to_string(), vec![2]).is_some());
    }
}
