#![crate_name="underscore"]
#![crate_id="underscore#0.0.0"]
#![crate_type="lib"]

pub struct U;

impl U {
    pub fn hello(self) -> int {
        1i
    }

    pub fn first<'a, T>(self, v: &'a Vec<T>) -> &'a T {
        v.get(0)
    }

    pub fn last<'a, T>(self, v: &'a Vec<T>) -> &'a T {
        let length = v.len();
        v.get(length - 1u)
    }
}
