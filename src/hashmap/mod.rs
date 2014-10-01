use std::hash::Hash;
use std::collections::HashMap;

pub struct Hashing;

impl<'a, K: Eq + Hash, V: Eq + Hash> Hashing {
    pub fn new() -> bool {
        true
    }

    pub fn pairs(&self, h: &HashMap<K, V>) -> Vec<(&'a K, &'a V)> {
        let mut pairs = Vec::new();
        for (key, value) in h.iter() {
            pairs.push((key, value));
        }

        return pairs;
    }
}
