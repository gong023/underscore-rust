#![crate_type="lib"]

pub mod collection;

pub struct Vect<T> {
    x: Vec<T>
}

impl<'a, T> Vect<T> {
    pub fn new(v: Vec<T>) -> Vect<T> {
        Vect { x: v }
    }

    pub fn first(&'a self) -> &'a T {
        &self.x[0]
    }

    /// TODO: fix interface to returns option
    pub fn last(&'a self) -> &'a T {
        &self.x[&self.x.len() - 1]
    }

    pub fn initial(&'a self, n: uint) -> Vec<&'a T> {
        let mut initialized = Vec::new();
        for i in range(0u, n) {
            initialized.push(&self.x[i])
        }

        return initialized;
    }

    pub fn rest(&'a self, n: uint) -> Vec<&'a T> {
        let mut rest = Vec::new();
        // デリファレンスしたらここで死ぬ気がする・・・
        for i in range(0u, *&self.x.len()) {
            if i < n { continue; }
            rest.push(&self.x[i])
        }

        return rest;
    }

    pub fn exists<T: PartialEq>(&'a self, x: &T) -> bool {
        for element in &self.x.iter() {
            if element.eq(x) { return true; }
        }

        return false;
    }
}
