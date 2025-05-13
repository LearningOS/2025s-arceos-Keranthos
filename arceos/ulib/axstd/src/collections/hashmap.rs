use core::hash::Hash;
use core::hash::Hasher;

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec};

struct Node<K: core::hash::Hash, V> {
    key: K,
    value: V,
    next: Option<Box<Node<K, V>>>,
}

pub struct HashMap<K: core::hash::Hash, V> {
    buckets: Vec<Option<Box<Node<K, V>>>>,
    length: usize,
    size: usize,
}

impl<K: core::hash::Hash, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            length: 0,
            size: 0,
        }
    }
    /*pub fn with_capacity(size: usize) -> Self {
        let length = next_power_of_two(size);
        Self {
            buckets: vec![None; length],
            length,
            size,
        }
    }*/
    pub fn insert(&mut self, key: K, value: V) {
        if self.length == 0 {
            self.length = next_power_of_two(1);
            self.buckets = vec![];
            self.buckets.resize_with(self.length, || None);
        }
        let index = hash(&key) % self.length;

        let new_node = Box::new(Node {
            key,
            value,
            next: self.buckets[index].take(),
        });

        self.buckets[index] = Some(new_node);

        self.size += 1;
        if self.size > 2 * self.length {
            self.resize()
        }
    }
    fn resize(&mut self) {
        let new_length = next_power_of_two(self.size.max(1) * 2);
        let mut new_buckets = vec![];
        new_buckets.resize_with(new_length, || None);

        for old_node in self.buckets.iter_mut() {
            while let Some(mut node) = old_node.take() {
                let index = hash(&node.key) % new_length;
                node.next = new_buckets[index].take();
                new_buckets[index] = Some(node);
            }
        }

        self.buckets = new_buckets;
        self.length = new_length;
    }
    pub fn iter(&self) -> HashMapIter<K, V> {
        HashMapIter {
            buckets: &self.buckets,
            bucket_index: 0,
            current_node: None,
        }
    }
}

pub struct HashMapIter<'a, K: core::hash::Hash, V> {
    buckets: &'a Vec<Option<Box<Node<K, V>>>>,
    bucket_index: usize,
    current_node: Option<&'a Node<K, V>>
}

impl<'a, K: core::hash::Hash, V> Iterator for HashMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current_node.take() {
            self.current_node = node.next.as_deref();
            return Some((&node.key, &node.value));
        }

        while self.bucket_index < self.buckets.len() {
            if let Some(ref node) = self.buckets[self.bucket_index] {
                self.current_node = node.as_ref().next.as_deref();
                self.bucket_index += 1;
                return Some((&node.key, &node.value));
            }
            self.bucket_index += 1;
        }

        None
    }
}

fn next_power_of_two(size: usize) -> usize {
    if size == 0 {
        return 0;
    }
    let mut power = 1;
    while power < size {
        power <<= 1;
    }
    power
}

fn hash<K: Hash>(key: &K) -> usize {
    let mut hasher = SimpleHasher::new();
    key.hash(&mut hasher);
    hasher.finish() as usize
}

struct SimpleHasher {
    state: u64,
}

impl SimpleHasher{
    fn new() -> Self {
        Self {
            state: 14695981039346656037,
        }
    }
}

impl Hasher for SimpleHasher {
    fn write(&mut self, bytes: &[u8]) {
        const FNV_PRIME: u64 = 1099511628211;
        for byte in bytes {
            self.state ^= *byte as u64;
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

/*impl Hash for i32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.to_le_bytes());
    }
}

impl Hash for usize {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.to_le_bytes());
    }
}

impl Hash for u32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.to_le_bytes());
    }
}

impl Hash for &str {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.as_bytes());
    }
}

impl Hash for &[u8] {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self);
    }
}*/
