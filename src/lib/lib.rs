#![crate_name="underscore"]
#![crate_id="underscore#0.0.0"]
#![crate_type="lib"]

pub struct Vect;

impl Vect {
    pub fn first<'a, T>(self, v: &'a Vec<T>) -> &'a T {
        v.get(0)
    }

    pub fn last<'a, T>(self, v: &'a Vec<T>) -> &'a T {
        let length = v.len();
        v.get(length - 1)
    }

    pub fn initial<'a, T>(self, v: &'a Vec<T>, n: uint) -> Vec<&'a T> {
        let mut initialized = Vec::new();
        for i in range(0u, n) {
            initialized.push(v.get(i))
        }

        return initialized;
    }

    pub fn rest<'a, T>(self, v: &'a Vec<T>, n: uint) -> Vec<&'a T> {
        let mut rest = Vec::new();
        for i in range(0u, v.len()) {
            if i < n { continue; }
            rest.push(v.get(i))
        }

        return rest;
    }
}
