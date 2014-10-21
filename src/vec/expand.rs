use vec::VecU;
use std::collections::TreeMap;

impl<T: PartialEq + Clone + Ord> VecU<T> for Vec<T>{
    fn first<'a>(&'a self) -> Option<&'a T> {
        self.as_slice().head()
    }

    fn without(self, values: &Vec<T>) -> Vec<T> {
        let mut without_elements = Vec::new();
        for element in self.into_iter() {
            if ! values.contains(&element) { without_elements.push(element) }
        }

        return without_elements;
    }

    fn intersection(self, intersec: &Vec<T>) -> Vec<T> {
        let mut intersected = Vec::new();
        for element in self.into_iter() {
            if intersec.contains(&element) { intersected.push(element) }
        }

        return intersected;
    }

    // FIXME: compare by Eq
    fn uniq(self) -> Vec<T> {
        let mut uniq = Vec::new();
        for element in self.into_iter() {
            if ! uniq.contains(&element) { uniq.push(element) }
        }

        return uniq;
    }

    fn index_of(&self, value: &T) -> Option<uint> {
        let mut index = 0u;
        for element in self.iter() {
            if element.eq(value) { return Some(index) }
            index += 1;
        }

        None
    }

    fn last_index_of(&self, value: &T) -> Option<uint> {
        let mut i = self.len();
        loop {
            i -= 1;
            if self[i].eq(value) { return Some(i); }
            if i == 0 { return None; }
        }
    }

    // FIXME: If duplicate keys exist, the last value wins.
    // FIXME: If values are shorter than keys, insert None.
    fn object<V: Clone>(self, value: Vec<V>) -> TreeMap<T, V> {
        let mut obj = TreeMap::new();
        for i in range(0u, self.len() - 1) {
            obj.insert(self[i].clone(), value[i].clone());
        }
        return obj;
    }

    fn reject(self, f: |value: &T| -> bool) -> Vec<T> {
        let mut rejected = Vec::new();
        for element in self.into_iter() {
            if ! f(&element) { rejected.push(element) }
        }
        return rejected;
    }
}

