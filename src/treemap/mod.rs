use std::collections::TreeMap;

pub mod expands;

/// UnderscoreTreeMap expands TreeMap
/// # Example
///
/// ```
/// pub use __::vec::UnderscoreTreeMap;
/// ```
pub trait UnderscoreTreeMap<K, V> {
    /// Returns a copy of the treemap where the keys have become the values and the values the keys.
    /// # Example
    ///
    /// ```
    /// let mut sample = TreeMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let inverted = sample.invert();
    /// // => TreeMap { 1u: 1i, 2u: 2i }
    /// ```
    fn invert(self) -> TreeMap<V, K>;

    /// Return a copy of the Treemap, filtered to only have values for the whitelisted keys.
    /// # Example
    ///
    /// ```
    /// let mut sample = TreeMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let picked = sample.pick(&vec!(1i));
    /// // => TreeMap { 1i: 1u }
    /// ```
    fn pick(self, keys: &Vec<K>) -> TreeMap<K, V>;

    /// Return a copy of the treemap, filtered to only have values for the vector whitelisted keys.
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
    /// let mut sample = TreeMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let picked = sample.pick_by_filter(sample_filter);
    /// // => TreeMap { 1i: 1u }
    /// ```
    fn pick_by_filter(self, f: |k: &K, v: &V| -> bool) -> TreeMap<K, V>;

    /// Return a copy of the treemap, filtered to omit the blacklisted keys (or array of keys).
    /// omit_by_filter filters keys to pick by function.
    ///
    /// ```
    /// let mut sample = TreeMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let omitted = sample.omit(&vec!(1i));
    /// // => TreeMap { 2i: 2u }
    /// ```
    fn omit(self, keys: &Vec<K>) -> TreeMap<K, V>;

    /// Return a copy of the treemap, filtered to omit the blacklisted keys (or array of keys).
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
    /// let mut sample = TreeMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let omitted = sample.omit_by_filter(sample_filter);
    /// // => TreeMap { 1u: 1i }
    /// ```
    fn omit_by_filter(self, f: |k: &K, v: &V| -> bool) -> TreeMap<K, V>;

    /// Fill in undefined properties in treemap with the first value present in the following list of defaults objects.
    /// # Example
    ///
    /// ```
    /// let mut origin = TreeMap::new();
    /// origin.insert(1i, 1u);
    /// origin.insert(2i, 2u);
    ///
    /// let mut appends = TreeMap::new();
    /// appends.insert(1i, 10000u);
    /// appends.insert(3i, 3u);
    ///
    /// let defaults = origin.defaults(appends);
    /// // => TreeMap { 1i: 1u, 2i: 2u, 3i: 3u }
    /// ```
    fn defaults(self, appends: TreeMap<K, V>) -> TreeMap<K, V>;

    /// Convert a treemap into a vector of (key, value) tuple pairs.
    /// # Example
    ///
    /// ```
    /// let mut sample = TreeMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let pairs = sample.pairs();
    /// // => Vec<(1i, 1u), (2i, 2u)>
    /// ```
    fn pairs<'a>(&'a self) -> Vec<(&'a K, &'a V)>;
}
