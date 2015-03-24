use std::collections::BTreeMap;

pub mod expand;

/// UnderscoreVec expands collections::vec::Vec
pub trait VecU<T> {
    /// Returns the first element of a vector as Option.
    /// # Example
    /// ```
    /// use underscore::vec::VecU;
    ///
    /// let sample = vec!(1i, 2, 3);
    /// assert_eq!(1i, *sample.first().unwrap());
    /// ```
    fn first<'a>(&'a self) -> Option<&'a T>;

    /// Returns a copy of the vector with all instances of the values removed.
    /// # Example
    /// ```
    /// use underscore::vec::VecU;
    ///
    /// let sample = vec!(1i, 2i, 2i);
    /// assert_eq!(vec!(2i, 2i), sample.without(&vec!(1i)));
    /// ```
    fn without(self, values: &Vec<T>) -> Vec<T>;

    /// Computes the list of values that are the intersection of argument vector.
    /// Each value in the result is present in each of the arrays.
    /// # Example
    /// ```
    /// use underscore::vec::VecU;
    ///
    /// let sample = vec!(1i, 2, 3);
    /// assert_eq!(vec!(2i, 3), sample.intersection(&vec!(2i, 3, 4)));
    /// ```
    fn intersection(self, intersec: &Vec<T>) -> Vec<T>;

    /// Produces a duplicate-free version of the vector.
    /// # Example
    ///
    /// ```
    /// use underscore::vec::VecU;
    ///
    /// let sample = vec!(0i, 1, 1, 1, 2, 2, 2, 3);
    /// assert_eq!(vec!(0i, 1, 2, 3), sample.uniq());
    /// ```
    fn uniq(self) -> Vec<T>;

    /// Returns the Option with index which value can be found in the vector.
    /// Pass true for is_sorted to use sorted vector.
    /// # Example
    ///
    /// ```
    /// use underscore::vec::VecU;
    ///
    /// let sample = vec!(3i, 2, 1);
    /// assert_eq!(2u, sample.index_of(&1i).unwrap());
    /// ```
    fn index_of(&self, value: &T) -> Option<uint>;

    /// Returns the Option with index which value can be found in the vector.
    /// # Example
    /// ```
    /// use underscore::vec::VecU;
    ///
    /// let sample = vec!(1i, 2, 3, 1);
    /// assert_eq!(3u, sample.last_index_of(&1i).unwrap());
    /// ```
    fn last_index_of(&self, value: &T) -> Option<uint>;

    /// Converts vector into BTreeMap.
    /// # Example
    /// ```
    /// use underscore::vec::VecU;
    ///
    /// let obj = vec!(0u, 1, 2, 3).object(vec!(0i, 1, 2, 3));
    /// //=> BTreeMap<0u, 0i><1u, 1i>...
    /// ```
    fn object<V: Clone>(self, value: Vec<V>) -> BTreeMap<T, V>;

    /// reject the values in Vector without the elements that the truth test (predicate) passes.
    /// The opposite of vec!().iter().filter();
    /// # Example
    /// ```
    /// use underscore::vec::VecU;
    ///
    /// let sample = vec!(1i, 2, 10);
    /// assert_eq!(vec!(10i), sample.reject(|&v| v < 10));
    /// ```
    fn reject<F: Fn(&T) -> bool>(self, f: F) -> Vec<T>;
}

