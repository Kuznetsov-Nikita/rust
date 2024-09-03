#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Entry<K, V> {
    key: K,
    value: V,
    next: usize,
    prev: usize,
}

impl<K, V> Entry<K, V> {
    pub fn new(key: K, value: V, next: usize, prev: usize) -> Self {
        Self {
            key,
            value,
            next,
            prev,
        }
    }
}

#[derive(Debug)]
pub struct LRUCache<K, V> {
    cache: HashMap<K, usize>,
    data: Vec<Entry<K, V>>,
    capacity: usize,
    head: usize,
    tail: usize,
    length: usize,
}

impl<K: Clone + Hash + Ord, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!();
        }
        Self {
            cache: HashMap::with_capacity(capacity),
            data: Vec::with_capacity(capacity),
            capacity,
            head: 0,
            tail: 0,
            length: 0,
        }
    }

    fn touch(&mut self, index: usize) {
        if self.head == index {
            return;
        }

        if self.length == 2 {
            std::mem::swap(&mut self.head, &mut self.tail);
            return;
        }

        let next = self.data[index].next;
        let prev = self.data[index].prev;

        if self.tail == index {
            self.tail = next;
        }

        self.data[prev].next = next;
        self.data[next].prev = prev;

        self.data[index].prev = self.head;
        self.data[self.head].next = index;

        self.data[index].next = self.tail;
        self.data[self.tail].prev = index;

        self.head = index;
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.cache.contains_key(key) {
            let index = self.cache[key];
            self.touch(index);
            Some(&(self.data[index].value))
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.cache.contains_key(&key) {
            let index = self.cache[&key];
            let mut value = value;
            std::mem::swap(&mut value, &mut self.data[index].value);
            self.touch(index);
            Some(value)
        } else {
            if self.length == self.capacity {
                self.cache.remove(&self.data[self.tail].key);
                self.cache.insert(key.clone(), self.tail);
                self.data[self.tail].key = key;
                self.data[self.tail].value = value;
                self.touch(self.tail);
            } else {
                self.cache.insert(key.clone(), self.length);
                let new_entry = Entry::new(key, value, self.tail, self.head);
                self.data.push(new_entry);
                self.data[self.head].next = self.length;
                self.data[self.tail].prev = self.length;
                self.head = self.length;
                self.length += 1;
            }

            None
        }
    }
}
