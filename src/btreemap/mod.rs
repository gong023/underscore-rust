use std::collections::BTreeMap;

pub mod expands;

/// BTreeMapU expands BTreeMap
pub trait BTreeMapU<K, V> {
    /// Returns a copy of the BTreeMap where the keys have become the values and the values the keys.
    /// # Example
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use underscore::btreemap::BTreeMapU;
    ///
    /// let mut sample = BTreeMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let inverted = sample.invert();
    /// // => BTreeMap { 1u: 1i, 2u: 2i }
    /// ```
    fn invert(self) -> BTreeMap<V, K>;

    /// Return a copy of the BTreeMap, filtered to only have values for the whitelisted keys.
    /// # Example
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use underscore::btreemap::BTreeMapU;
    ///
    /// let mut sample = BTreeMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let picked = sample.pick(&vec!(1i));
    /// // => BTreeMap { 1i: 1u }
    /// ```
    fn pick(self, keys: &Vec<K>) -> BTreeMap<K, V>;

    /// Return a copy of the BTreeMap, filtered to only have values for the vector whitelisted keys.
    /// pick_by_filter filters keys to pick by function.
    /// # Example
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use underscore::btreemap::BTreeMapU;
    ///
    /// #[allow(unused_variable)]
    /// fn sample_filter(x: &int, y: &uint) -> bool {
    ///     if *x == 1 { return true; }
    ///     return false;
    /// }
    ///
    /// let mut sample = BTreeMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let picked = sample.pick_by_filter(sample_filter);
    /// // => BTreeMap { 1i: 1u }
    /// ```
    fn pick_by_filter<F: Fn(&K, &V) -> bool>(self, f: F) -> BTreeMap<K,V>;

    /// Return a copy of the BTreeMap, filtered to omit the blacklisted keys (or array of keys).
    /// omit_by_filter filters keys to pick by function.
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use underscore::btreemap::BTreeMapU;
    ///
    /// let mut sample = BTreeMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let omitted = sample.omit(&vec!(1i));
    /// // => BTreeMap { 2i: 2u }
    /// ```
    fn omit(self, keys: &Vec<K>) -> BTreeMap<K, V>;

    /// Return a copy of the BTreeMap, filtered to omit the blacklisted keys (or array of keys).
    /// omit_by_filter filters keys to pick by function.
    /// # Example
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use underscore::btreemap::BTreeMapU;
    ///
    /// #[allow(unused_variable)]
    /// fn sample_filter(x: &int, y: &uint) -> bool {
    ///     if *x == 1 { return true; }
    ///     return false;
    /// }
    ///
    /// let mut sample = BTreeMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let omitted = sample.omit_by_filter(sample_filter);
    /// // => BTreeMap { 1u: 1i }
    /// ```
    fn omit_by_filter<F: Fn(&K, &V) -> bool>(self, f: F) -> BTreeMap<K, V>;

    /// Fill in undefined properties in BTreeMap with the first value present in the following list of defaults objects.
    /// # Example
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use underscore::btreemap::BTreeMapU;
    ///
    /// let mut origin = BTreeMap::new();
    /// origin.insert(1i, 1u);
    /// origin.insert(2i, 2u);
    ///
    /// let mut appends = BTreeMap::new();
    /// appends.insert(1i, 10000u);
    /// appends.insert(3i, 3u);
    ///
    /// let defaults = origin.defaults(appends);
    /// // => BTreeMap { 1i: 1u, 2i: 2u, 3i: 3u }
    /// ```
    fn defaults(self, appends: BTreeMap<K, V>) -> BTreeMap<K, V>;

    /// Convert a BTreeMap into a vector of (key, value) tuple pairs.
    /// # Example
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use underscore::btreemap::BTreeMapU;
    ///
    /// let mut sample = BTreeMap::new();
    /// sample.insert(1i, 1u);
    /// sample.insert(2i, 2u);
    ///
    /// let pairs = sample.pairs();
    /// // => Vec<(1i, 1u), (2i, 2u)>
    /// ```
    fn pairs<'a>(&'a self) -> Vec<(&'a K, &'a V)>;
}
