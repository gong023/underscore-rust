use std::collections::TreeMap;
use treemap::TreeMapU;

impl<K: Ord + Clone, V: Ord + Clone> TreeMapU<K, V> for TreeMap<K, V> {

    fn invert(self) -> TreeMap<V, K> {
        let mut invert = TreeMap::new();
        for (key, value) in self.into_iter() {
            invert.insert(value, key);
        }
        return invert;
    }

    fn pick(self, keys: &Vec<K>) -> TreeMap<K, V> {
        let mut picked = TreeMap::new();
        for element in keys.iter() {
            match self.find(element) {
                Some(value) => { picked.insert(element.clone(), value.clone()); },
                None => fail!("not found key"),
            }
        }

        return picked;
    }

    fn pick_by_filter(self, f: |k: &K, v: &V| -> bool) -> TreeMap<K, V> {
        let mut picked = TreeMap::new();
        for (key, value) in self.into_iter() {
            if f(&key, &value) { picked.insert(key, value); }
        }
        return picked;
    }

    fn omit(self, keys: &Vec<K>) -> TreeMap<K, V> {
        let mut omitted = TreeMap::new();
        for (key, value) in self.into_iter() {
            if ! keys.contains(&key) { omitted.insert(key, value); }
        }
        return omitted;
    }

    fn omit_by_filter(self, f: |k: &K, v: &V| -> bool) -> TreeMap<K, V> {
        let mut omitted = TreeMap::new();
        for (key, value) in self.into_iter() {
            if ! f(&key, &value) { omitted.insert(key, value); }
        }
        return omitted;
    }

    fn defaults(self, appends: TreeMap<K, V>) -> TreeMap<K, V> {
        let mut origin = self.clone();
        for (key, value) in appends.into_iter() {
            if ! origin.contains_key(&key) { origin.insert(key, value); }
        }

        return origin;
    }

    fn pairs<'a>(&'a self) -> Vec<(&'a K, &'a V)> {
        let mut pairs = Vec::new();
        for (key, value) in self.iter() {
            pairs.push((key, value));
        }

        return pairs;
    }
}
