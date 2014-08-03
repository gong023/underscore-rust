#![crate_name="underscore"]
#![crate_id="underscore#0.0.0"]
#![crate_type="lib"]

pub struct U;

impl U {
    pub fn hello(self) -> int {
        1i
    }

//    pub fn first<T>(self, arr: &[T]) -> Option<&T> {
//        arr.get(0u)
//    }

    pub fn first(self, arr: &[int]) -> int {
        arr[0]
    }
}
