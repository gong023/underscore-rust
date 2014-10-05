use std::hash::Hash;
use std::collections::HashMap;

pub struct Hashing<K: Eq + Hash, V: Eq + Hash> {
    x: HashMap<K, V>
}

impl<'a, K: Eq + Hash, V: Eq + Hash> Hashing<K, V> {
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

//    pub fn pick<T>(self, keys: &Vec<T>) -> HashMap<K, V> {
//        let mut picked = HashMap::new();
//        for (key, value) in self.x.into_iter() {
//        }
//    }
}
