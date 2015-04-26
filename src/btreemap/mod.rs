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
    /// sample.insert(1isize, 1usize);
    /// sample.insert(2isize, 2usize);
    ///
    /// let inverted = sample.invert();
    /// // => BTreeMap { 1usize: 1isize, 2usize: 2isize }
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
    /// sample.insert(1isize, 1usize);
    /// sample.insert(2isize, 2usize);
    ///
    /// let picked = sample.pick(&vec!(1isize));
    /// // => BTreeMap { 1isize: 1usize }
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
    /// fn sample_filter(x: &isize, y: &usize) -> bool {
    ///     if *x == 1 { return true; }
    ///     return false;
    /// }
    ///
    /// let mut sample = BTreeMap::new();
    /// sample.insert(1isize, 1usize);
    /// sample.insert(2isize, 2usize);
    ///
    /// let picked = sample.pick_by_filter(sample_filter);
    /// // => BTreeMap { 1isize: 1usize }
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
    /// sample.insert(1isize, 1usize);
    /// sample.insert(2isize, 2usize);
    ///
    /// let omitted = sample.omit(&vec!(1isize));
    /// // => BTreeMap { 2isize: 2usize }
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
    /// fn sample_filter(x: &isize, y: &usize) -> bool {
    ///     if *x == 1 { return true; }
    ///     return false;
    /// }
    ///
    /// let mut sample = BTreeMap::new();
    /// sample.insert(1isize, 1usize);
    /// sample.insert(2isize, 2usize);
    ///
    /// let omitted = sample.omit_by_filter(sample_filter);
    /// // => BTreeMap { 1usize: 1isize }
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
    /// origin.insert(1isize, 1usize);
    /// origin.insert(2isize, 2usize);
    ///
    /// let mut appends = BTreeMap::new();
    /// appends.insert(1isize, 10000usize);
    /// appends.insert(3isize, 3usize);
    ///
    /// let defaults = origin.defaults(appends);
    /// // => BTreeMap { 1isize: 1usize, 2isize: 2usize, 3isize: 3usize }
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
    /// sample.insert(1isize, 1usize);
    /// sample.insert(2isize, 2usize);
    ///
    /// let pairs = sample.pairs();
    /// // => Vec<(1isize, 1usize), (2isize, 2usize)>
    /// ```
    fn pairs<'a>(&'a self) -> Vec<(&'a K, &'a V)>;
}
