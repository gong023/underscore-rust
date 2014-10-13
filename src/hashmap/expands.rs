use std::hash::Hash;
use std::collections::HashMap;
use hashmap::UnderscoreHashMap;

impl<'a, K: Eq + Hash + Clone, V: Eq + Hash + Clone> UnderscoreHashMap<K, V> for HashMap<K, V> {

    /// Returns a copy of the hashmap where the keys have become the values and the values the keys.
    /// usage:
    ///
    /// ```
    /// let mut sample = HashMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let inverted = sample.invert();
    /// // => HashMap { 1u: 1i, 2u: 2i }
    /// ```
    fn invert(self) -> HashMap<V, K> {
        let mut invert = HashMap::new();
        for (key, value) in self.into_iter() {
            invert.insert(value, key);
        }

        return invert;
    }

    /// Return a copy of the hashmap, filtered to only have values for the whitelisted keys.
    /// usage:
    ///
    /// ```
    /// let mut sample = HashMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let picked = sample.pick(vec!(1i));
    /// // => HashMap { 1u: 1i }
    /// ```
    fn pick(self, keys: Vec<K>) -> HashMap<K, V> {
        let mut picked = HashMap::new();
        for element in keys.into_iter() {
            if self.contains_key(&element) {
                let v = self.get_copy(&element);
                picked.insert(element, v);
            }
        }

        return picked;
    }

    /// Return a copy of the hashmap, filtered to only have values for the vector whitelisted keys.
    /// pick_by_filter filters keys to pick by function.
    /// usage:
    ///
    /// ```
    /// #[allow(unused_variable)]
    /// fn sample_filter(x: &int, y: &uint) -> bool {
    ///     if *x == 1 { return true; }
    ///     return false;
    /// }
    ///
    /// let mut sample = HashMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let picked = sample.pick_by_filter(sample_filter);
    /// // => HashMap { 1u: 1i }
    /// ```
    fn pick_by_filter(self, f: |k: &K, v: &V| -> bool) -> HashMap<K, V> {
        let mut picked = HashMap::new();
        for key in self.keys() {
            let cp_key = key.clone();
            let cp_val = self.get_copy(key);
            if f(&cp_key, &cp_val) { picked.insert(cp_key, cp_val); }
        }
        return picked;
    }

    /// Return a copy of the hashmap, filtered to omit the blacklisted keys (or array of keys).
    /// usage:
    ///
    /// ```
    /// let mut sample = HashMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let omitted = sample.omit(vec!(1i));
    /// // => HashMap { 2u: 2i }
    /// ```
    fn omit(self, keys: Vec<K>) -> HashMap<K, V> {
        let mut omitted = HashMap::new();
        for (key, value) in self.into_iter() {
            if ! keys.contains(&key) { omitted.insert(key, value); }
        }

        return omitted;
    }

    /// Return a copy of the hashmap, filtered to omit the blacklisted keys (or array of keys).
    /// omit_by_filter filters keys to pick by function.
    /// usage:
    ///
    /// ```
    /// #[allow(unused_variable)]
    /// fn sample_filter(x: &int, y: &uint) -> bool {
    ///     if *x == 1 { return true; }
    ///     return false;
    /// }
    ///
    /// let mut sample = HashMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let inverted = sample.omit_by_filter(sample_filter);
    /// // => HashMap { 2u: 2i }
    /// ```
    fn omit_by_filter(self, f: |k: &K, v: &V| -> bool) -> HashMap<K, V> {
        let mut omitted = HashMap::new();
        for key in self.keys() {
            let cp_key = key.clone();
            let cp_val = self.get_copy(key);
            if ! f(&cp_key, &cp_val) { omitted.insert(cp_key, cp_val); }
        }
        return omitted;
    }

    /// Fill in undefined properties in hashmap with the first value present in the following list of defaults objects.
    /// usage:
    ///
    /// ```
    /// let mut origin = HashMap::new();
    /// origin.insert(1i, 1u);
    /// origin.insert(2i, 2u);
    ///
    /// let mut appends = HashMap::new();
    /// appends.insert(1i, 10000u);
    /// appends.insert(3i, 3u);
    ///
    /// let defaults = __::hashmap::Hashing::new(origin).defaults(appends);
    /// // => HashMap { 1i, 1u, 2u: 2i, 3i: 3u }
    /// ```
    fn defaults(self, appends: HashMap<K, V>) -> HashMap<K, V> {
        let mut origin = self.clone();
        for (key, value) in appends.into_iter() {
            if ! origin.contains_key(&key) { origin.insert(key, value); }
        }

        return origin;
    }
}
