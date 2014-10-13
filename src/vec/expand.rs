use vec::UnderscoreVec;
use std::collections::TreeMap;

impl<T: PartialEq + Clone + Ord> UnderscoreVec<T> for Vec<T>{
    /// Returns the first element of a vector as Option.
    /// usage:
    ///
    /// ```
    /// let sample = vec!(1i, 2, 3);
    /// assert_eq!(1i, *sample.first().unwrap());
    /// ```
    fn first<'a>(&'a self) -> Option<&'a T> {
        self.as_slice().head()
    }

    /// Returns a copy of the vector with all instances of the values removed.
    /// usage:
    ///
    /// ```
    /// let sample = vec!(1i, 2i, 2i);
    /// assert_eq!(vec!(2i, 2i), sample.without(&vec!(1i)));
    /// ```
    fn without(self, values: &Vec<T>) -> Vec<T> {
        let mut without_elements = Vec::new();
        for element in self.into_iter() {
            if ! values.contains(&element) { without_elements.push(element) }
        }

        return without_elements;
    }

    /// Computes the list of values that are the intersection of argument vector.
    /// Each value in the result is present in each of the arrays.
    /// usage:
    ///
    /// ```
    /// let sample = vec!(1i, 2, 3);
    /// assert_eq!(vec!(2i, 3), sample.intersection(&vec!(2i, 3, 4)));
    /// ```
    fn intersection(self, intersec: &Vec<T>) -> Vec<T> {
        let mut intersected = Vec::new();
        for element in self.into_iter() {
            if intersec.contains(&element) { intersected.push(element) }
        }

        return intersected;
    }

    /// Produces a duplicate-free version of the vector.
    /// FIXME: compare by Eq
    /// usage:
    ///
    /// ```
    /// let sample = vec!(0i, 1, 1, 1, 2, 2, 2, 3);
    /// assert_eq!(vec!(0u, 1, 2, 3), sample.uniq());
    /// ```
    fn uniq(self) -> Vec<T> {
        let mut uniq = Vec::new();
        for element in self.into_iter() {
            if ! uniq.contains(&element) { uniq.push(element) }
        }

        return uniq;
    }

    /// Returns the Option with index which value can be found in the vector.
    /// Pass true for is_sorted to use sorted vector.
    /// usage:
    ///
    /// ```
    /// let sample = vec!(3i, 2, 1);
    /// assert_eq!(2u, sample.index_of(&1i).unwrap());
    /// ```
    fn index_of(&self, value: &T) -> Option<uint> {
        let mut index = 0u;
        for element in self.iter() {
            if element.eq(value) { return Some(index) }
            index += 1;
        }

        None
    }

    /// Returns the Option with index which value can be found in the vector.
    /// usage:
    ///
    /// ```
    /// let sample = vec!(1i, 2, 3, 1);
    /// assert_eq!(3u, sample.last_index_of(&1i).unwrap());
    /// ```
    fn last_index_of(&self, value: &T) -> Option<uint> {
        let mut i = self.len();
        loop {
            i -= 1;
            if self[i].eq(value) { return Some(i); }
            if i == 0 { return None; }
        }
    }

    /// Converts vector into treemap.
    /// FIXME: If duplicate keys exist, the last value wins.
    /// FIXME: If values are shorter than keys, insert None.
    /// usage:
    ///
    /// ```
    /// let obj = vec!(0u, 1, 2, 3).object(vec!(0i, 1, 2, 3));
    /// => TreeMap<0u, 0i><1u, 1i>...
    /// ```
    fn object<V: Clone>(self, value: Vec<V>) -> TreeMap<T, V> {
        let mut obj = TreeMap::new();
        for i in range(0u, self.len() - 1) {
            obj.insert(self[i].clone(), value[i].clone());
        }
        return obj;
    }
}

