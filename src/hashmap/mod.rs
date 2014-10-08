use std::hash::Hash;
use std::collections::HashMap;

pub struct Hashing<K, V> {
    x: HashMap<K, V>
}

impl<'a, K: Eq + Hash, V: Eq + Hash + Clone> Hashing<K, V> {
    pub fn new(h: HashMap<K, V>) -> Hashing<K, V> {
        Hashing { x: h }
    }

    pub fn pairs(self) -> Vec<(K, V)> {
        let mut pairs = Vec::new();
        for (key, value) in self.x.into_iter() {
            pairs.push((key, value));
        }

        return pairs;
    }

    pub fn invert(self) -> HashMap<V, K> {
        let mut invert = HashMap::new();
        for (key, value) in self.x.into_iter() {
            invert.insert(value, key);
        }

        return invert;
    }

    pub fn pick(self, keys: Vec<K>) -> HashMap<K, V> {
        let mut picked = HashMap::new();
        for element in keys.into_iter() {
            if self.x.contains_key(&element) {
                let v = self.x.get_copy(&element);
                picked.insert(element, v);
            }
        }

        return picked;
    }
}
