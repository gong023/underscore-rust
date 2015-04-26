use std::collections::HashMap;

pub mod expands;

/// UnderscoreHashMap expands HashMap
pub trait HashMapU<K, V> {
    /// Returns a copy of the hashmap where the keys have become the values and the values the keys.
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use underscore::hashmap::HashMapU;
    ///
    /// let mut sample = HashMap::new();
    /// sample.insert(1isize, 1usize);
    /// sample.insert(2isize, 2usize);
    ///
    /// let inverted = sample.invert();
    /// // => HashMap { 1usize: 1isize, 2usize: 2isize }
    /// ```
    fn invert(self) -> HashMap<V, K>;

    /// Return a copy of the hashmap, filtered to only have values for the whitelisted keys.
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use underscore::hashmap::HashMapU;
    ///
    /// let mut sample = HashMap::new();
    /// sample.insert(1isize, 1usize);
    /// sample.insert(2isize, 2usize);
    ///
    /// let picked = sample.pick(vec!(1isize));
    /// // => HashMap { 1usize: 1isize }
    /// ```
    fn pick(self, keys: Vec<K>) -> HashMap<K, V>;

    /// Return a copy of the hashmap, filtered to only have values for the vector whitelisted keys.
    /// pick_by_filter filters keys to pick by function.
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use underscore::hashmap::HashMapU;
    ///
    /// #[allow(unused_variable)]
    /// fn sample_filter(x: &isize, y: &usize) -> bool {
    ///     if *x == 1 { return true; }
    ///     return false;
    /// }
    ///
    /// let mut sample = HashMap::new();
    /// sample.insert(1isize, 1usize);
    /// sample.insert(2isize, 2usize);
    ///
    /// let picked = sample.pick_by_filter(sample_filter);
    /// // => HashMap { 1usize: 1isize }
    /// ```
    fn pick_by_filter<F: Fn(&K, &V) -> bool>(self, f: F) -> HashMap<K, V>;

    /// Return a copy of the hashmap, filtered to omit the blacklisted keys (or array of keys).
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use underscore::hashmap::HashMapU;
    ///
    /// let mut sample = HashMap::new();
    /// sample.insert(1isize, 1usize);
    /// sample.insert(2isize, 2usize);
    ///
    /// let omitted = sample.omit(vec!(1isize));
    /// // => HashMap { 2usize: 2isize }
    /// ```
    fn omit(self, keys: Vec<K>) -> HashMap<K, V>;

    /// Return a copy of the hashmap, filtered to omit the blacklisted keys (or array of keys).
    /// omit_by_filter filters keys to pick by function.
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use underscore::hashmap::HashMapU;
    ///
    /// #[allow(unused_variable)]
    /// fn sample_filter(x: &isize, y: &usize) -> bool {
    ///     if *x == 1 { return true; }
    ///     return false;
    /// }
    ///
    /// let mut sample = HashMap::new();
    /// sample.insert(1isize, 1usize);
    /// sample.insert(2isize, 2usize);
    ///
    /// let inverted = sample.omit_by_filter(sample_filter);
    /// // => HashMap { 2usize: 2isize }
    /// ```
    fn omit_by_filter<F: Fn(&K, &V) -> bool>(self, f: F) -> HashMap<K, V>;

    /// Fill in undefined properties in hashmap with the first value present in the following list of defaults objects.
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use underscore::hashmap::HashMapU;
    ///
    /// let mut origin = HashMap::new();
    /// origin.insert(1isize, 1usize);
    /// origin.insert(2isize, 2usize);
    ///
    /// let mut appends = HashMap::new();
    /// appends.insert(1isize, 10000usize);
    /// appends.insert(3isize, 3usize);
    ///
    /// let defaults = origin.defaults(appends);
    /// // => HashMap { 1isize: 1usize, 2isize: 2usize, 3isize: 3usize }
    /// ```
    fn defaults(self, appends: HashMap<K, V>) -> HashMap<K, V>;

    // needs #![feature(unboxed_closures, unboxed_closure_sugar)] and the are still experimental
    // fn property(self, key: K) -> Box<|&:|:'static -> V>;
}
