pub struct Vect<T> {
    x: Vec<T>
}

impl<'a, T: PartialEq + Clone> Vect<T> {

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
        if *&self.x.as_slice().is_empty() {
            None
        } else {
            Some(&self.x[0])
        }
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
}

