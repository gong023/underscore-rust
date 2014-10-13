#![crate_type="lib"]

pub mod vec;
pub mod hashmap;

pub struct Iter;

impl Iter {
    pub fn each<'a, T>(self, v: &'a Vec<T>, func: |x: &'a T, index: uint|) {
        let mut i = 0u;
        loop {
            match i <= v.len() {
                true  => func(&v[i], i),
                false => break
            }
            i += 1;
        }
    }
}
