use std::collections::HashMap;

pub mod expands;

/// UnderscoreHashMap expands HashMap
/// # Example
///
/// ```
/// pub use __::vec::UnderscoreHashMap;
/// ```
pub trait HashMapU<K, V> {
    /// Returns a copy of the hashmap where the keys have become the values and the values the keys.
    /// # Example
    ///
    /// ```
    /// let mut sample = HashMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let inverted = sample.invert();
    /// // => HashMap { 1u: 1i, 2u: 2i }
    /// ```
    fn invert(self) -> HashMap<V, K>;

    /// Return a copy of the hashmap, filtered to only have values for the whitelisted keys.
    /// # Example
    ///
    /// ```
    /// let mut sample = HashMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let picked = sample.pick(vec!(1i));
    /// // => HashMap { 1u: 1i }
    /// ```
    fn pick(self, keys: Vec<K>) -> HashMap<K, V>;

    /// Return a copy of the hashmap, filtered to only have values for the vector whitelisted keys.
    /// pick_by_filter filters keys to pick by function.
    /// # Example
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
    fn pick_by_filter(self, f: |k: &K, v: &V| -> bool) -> HashMap<K, V>;

    /// Return a copy of the hashmap, filtered to omit the blacklisted keys (or array of keys).
    /// # Example
    ///
    /// ```
    /// let mut sample = HashMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let omitted = sample.omit(vec!(1i));
    /// // => HashMap { 2u: 2i }
    /// ```
    fn omit(self, keys: Vec<K>) -> HashMap<K, V>;

    /// Return a copy of the hashmap, filtered to omit the blacklisted keys (or array of keys).
    /// omit_by_filter filters keys to pick by function.
    /// # Example
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
    fn omit_by_filter(self, f: |k: &K, v: &V| -> bool) -> HashMap<K, V>;

    /// Fill in undefined properties in hashmap with the first value present in the following list of defaults objects.
    /// # Example
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
    /// let defaults = origin.defaults(appends);
    /// // => HashMap { 1i: 1u, 2i: 2u, 3i: 3u }
    /// ```
    fn defaults(self, appends: HashMap<K, V>) -> HashMap<K, V>;

    // needs #![feature(unboxed_closures, unboxed_closure_sugar)] and the are still experimental
    // fn property(self, key: K) -> Box<|&:|:'static -> V>;
}
