use std::collections::TreeMap;

pub mod expand;

/// UnderscoreVec expands collections::vec::Vec
/// # Example
/// ```
/// pub use __::vec::UnderscoreVec;
/// ```
pub trait UnderscoreVec<T> {
    /// Returns the first element of a vector as Option.
    /// # Example
    /// ```
    /// let sample = vec!(1i, 2, 3);
    /// assert_eq!(1i, *sample.first().unwrap());
    /// ```
    fn first<'a>(&'a self) -> Option<&'a T>;

    /// Returns a copy of the vector with all instances of the values removed.
    /// # Example
    /// ```
    /// let sample = vec!(1i, 2i, 2i);
    /// assert_eq!(vec!(2i, 2i), sample.without(&vec!(1i)));
    /// ```
    fn without(self, values: &Vec<T>) -> Vec<T>;

    /// Computes the list of values that are the intersection of argument vector.
    /// Each value in the result is present in each of the arrays.
    /// # Example
    /// ```
    /// let sample = vec!(1i, 2, 3);
    /// assert_eq!(vec!(2i, 3), sample.intersection(&vec!(2i, 3, 4)));
    /// ```
    fn intersection(self, intersec: &Vec<T>) -> Vec<T>;

    /// Produces a duplicate-free version of the vector.
    /// # Example
    ///
    /// ```
    /// let sample = vec!(0i, 1, 1, 1, 2, 2, 2, 3);
    /// assert_eq!(vec!(0u, 1, 2, 3), sample.uniq());
    /// ```
    fn uniq(self) -> Vec<T>;

    /// Returns the Option with index which value can be found in the vector.
    /// Pass true for is_sorted to use sorted vector.
    /// # Example
    ///
    /// ```
    /// let sample = vec!(3i, 2, 1);
    /// assert_eq!(2u, sample.index_of(&1i).unwrap());
    /// ```
    fn index_of(&self, value: &T) -> Option<uint>;

    /// Returns the Option with index which value can be found in the vector.
    /// # Example
    /// ```
    /// let sample = vec!(1i, 2, 3, 1);
    /// assert_eq!(3u, sample.last_index_of(&1i).unwrap());
    /// ```
    fn last_index_of(&self, value: &T) -> Option<uint>;

    /// Converts vector into treemap.
    /// # Example
    /// ```
    /// let obj = vec!(0u, 1, 2, 3).object(vec!(0i, 1, 2, 3));
    /// => TreeMap<0u, 0i><1u, 1i>...
    /// ```
    fn object<V: Clone>(self, value: Vec<V>) -> TreeMap<T, V>;

    /// reject the values in Vector without the elements that the truth test (predicate) passes.
    /// The opposite of vec!().iter().filter();
    /// # Example
    /// ```
    /// let sample = vec!(1i, 2, 10);
    /// assert_eq!(vec!(10i), sample.reject(|&v| x < 10));
    /// ```
    fn reject(self, f: |value: &T| -> bool) -> Vec<T>;
}

