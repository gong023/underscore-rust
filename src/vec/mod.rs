use std::collections::TreeMap;

pub mod expand;

/// UnderscoreVec expands collections::vec::Vec
/// usage:
///
/// ```
/// pub use __::vec::UnderscoreVec;
/// ```
pub trait UnderscoreVec<T> {
    fn first<'a>(&'a self) -> Option<&'a T>;

    fn without(self, values: &Vec<T>) -> Vec<T>;

    fn intersection(self, intersec: &Vec<T>) -> Vec<T>;

    fn uniq(self) -> Vec<T>;

    fn index_of(&self, value: &T) -> Option<uint>;

    fn last_index_of(&self, value: &T) -> Option<uint>;

    fn object<V: Clone>(self, value: Vec<V>) -> TreeMap<T, V>;
}

