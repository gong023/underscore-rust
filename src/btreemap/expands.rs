use std::collections::BTreeMap;
use btreemap::BTreeMapU;

impl<K: Ord + Clone, V: Ord + Clone> BTreeMapU<K, V> for BTreeMap<K, V> {

    fn invert(self) -> BTreeMap<V, K> {
        let mut invert = BTreeMap::new();
        for (key, value) in self.into_iter() {
            invert.insert(value, key);
        }
        return invert;
    }

    fn pick(self, keys: &Vec<K>) -> BTreeMap<K, V> {
        let mut picked = BTreeMap::new();
        for element in keys.iter() {
            match self.get(element) {
                Some(value) => { picked.insert(element.clone(), value.clone()); },
                None => {},
            }
        }

        return picked;
    }

    fn pick_by_filter<F: Fn(&K, &V) -> bool>(self, f: F) -> BTreeMap<K, V> {
        let mut picked = BTreeMap::new();
        for (key, value) in self.into_iter() {
            if f(&key, &value) { picked.insert(key, value); }
        }
        return picked;
    }

    fn omit(self, keys: &Vec<K>) -> BTreeMap<K, V> {
        let mut omitted = BTreeMap::new();
        for (key, value) in self.into_iter() {
            if ! keys.contains(&key) { omitted.insert(key, value); }
        }
        return omitted;
    }

    fn omit_by_filter<F: Fn(&K, &V) -> bool>(self, f: F) -> HashMap<K, V> {
        let mut omitted = BTreeMap::new();
        for (key, value) in self.into_iter() {
            if ! f(&key, &value) { omitted.insert(key, value); }
        }
        return omitted;
    }

    fn defaults(self, appends: BTreeMap<K, V>) -> BTreeMap<K, V> {
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
