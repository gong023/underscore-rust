use std::collections::TreeMap;

pub struct Vect<T> {
    x: Vec<T>
}

impl<'a, T: PartialEq + Clone + Ord> Vect<T> {

    /// usage:
    ///
    /// ```
    /// extern crate __;
    ///
    /// // you can get Vect struct by `new`.
    /// __::vec::Vect::new(vec!(1i, 2, 3));
    /// ```
    pub fn new(v: Vec<T>) -> Vect<T> {
        Vect { x: v }
    }

    /// Returns the first element of a vector as Option.
    /// usage:
    ///
    /// ```
    /// let sample = vec!(1i, 2, 3);
    ///
    /// assert_eq!(1i, *__::vec::Vect::new(sample).first().unwrap());
    /// ```
    pub fn first(&'a self) -> Option<&'a T> {
        self.x.as_slice().head()
    }

    /// Returns a copy of the vector with all instances of the values removed.
    /// usage:
    ///
    /// ```
    /// let vec_int = vec!(1i, 2i, 2i);
    ///
    /// __::vec::Vect::new(vec_int).without(&vec!(1i));
    /// // => vector [2i, 2i]
    /// ```
    pub fn without(self, values: &Vec<T>) -> Vec<T> {
        let mut without_elements = Vec::new();
        for element in self.x.into_iter() {
            if ! values.contains(&element) { without_elements.push(element) }
        }

        return without_elements;
    }

    /// Computes the list of values that are the intersection of argument vector.
    /// Each value in the result is present in each of the arrays.
    /// usage:
    ///
    /// ```
    /// let vec_int = vec!(1i, 2, 3);
    ///
    /// __::vec::Vect::new(vec_int).intersection(&vec!(2i, 3, 4));
    /// // => vector [2i, 3i]
    /// ```
    pub fn intersection(self, intersec: &Vec<T>) -> Vec<T> {
        let mut intersected = Vec::new();
        for element in self.x.into_iter() {
            if intersec.contains(&element) { intersected.push(element) }
        }

        return intersected;
    }

    /// Produces a duplicate-free version of the vector.
    /// FIXME: compare by Eq
    /// usage:
    ///
    /// ```
    /// let vec_int = vec!(0i, 1, 1, 1, 2, 2, 2, 3);
    ///
    /// __::vec::Vect::new(vec_int).uniq();
    /// // => vector [0i, 1, 2, 3]
    /// ```
    pub fn uniq(self) -> Vec<T> {
        let mut uniq = Vec::new();
        for element in self.x.into_iter() {
            if ! uniq.contains(&element) { uniq.push(element) }
        }

        return uniq;
    }

    /// Returns the Option with index which value can be found in the vector.
    /// Pass true for is_sorted to use sorted vector.
    /// FIXME: do not take ownership
    /// usage:
    ///
    /// ```
    /// let vec_int = vec!(3i, 2, 1);
    ///
    /// __::vec::Vect::new(vec_int).index_of(&1i, false).unwrap();
    /// // => 2u
    /// __::vec::Vect::new(vec_int).index_of(&1i, true).unwrap();
    /// // => 0u
    /// __::vec::Vect::new(vec!(3i, 2, 1)).index_of(&4i, false);
    /// // => None
    /// ```
    pub fn index_of(self, value: &T, is_sorted: bool) -> Option<uint> {
        let mut copy = self.x.clone();
        if is_sorted { copy.sort() }

        let mut index = 0u;
        for element in copy.iter() {
            if element.eq(value) { return Some(index) }
            index += 1;
        }

        None
    }

    /// Returns the Option with index which value can be found in the vector.
    /// FIXME: do not take ownership
    /// usage:
    ///
    /// ```
    /// let vec_int = vec!(1i, 2, 3, 1);
    ///
    /// __::vec::Vect::new(vec_int).last_index_of(&1i).unwrap();
    /// // => 3u
    /// __::vec::Vect::new(vec_int).last_index_of(&4i);
    /// // => None
    /// ```
    pub fn last_index_of(self, value: &T) -> Option<uint> {
        let mut copy = self.x.clone();
        copy.reverse();

        let mut index = copy.len() - 1;
        for element in copy.iter() {
            if element.eq(value) { return Some(index) }
            index -= 1;
        }

        None
    }

    /// Converts vector into hashmap. If duplicate keys exist, the last value wins.
    /// usage:
    ///
    /// ```
    /// let keys = vec!(0u, 1, 2, 3);
    /// let values = vec!(0i, 1, 2, 3);
    ///
    /// let obj = __::vec::Vect::new(keys).object(values);
    /// => TreeMap<0u, 0i>...
    /// ```
    pub fn object<V: Clone>(self, value: Vec<V>) -> TreeMap<T, V> {
        let mut obj = TreeMap::new();
        for i in range(0u, self.x.len() - 1) {
            obj.insert(self.x[i].clone(), value[i].clone());
        }
        return obj;
    }
}

