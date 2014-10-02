pub struct Vect<T> {
    x: Vec<T>
}

impl<'a, T: PartialEq + Clone> Vect<T> {
    pub fn new(v: Vec<T>) -> Vect<T> {
        Vect { x: v }
    }

    pub fn exists(x: &T, v: &Vec<T>) -> bool {
        for element in v.iter() {
            if element.eq(x) { return true; }
        }

        return false;
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

    pub fn without(&'a self, values: &Vec<T>) -> Vec<&'a T> {
        let mut without_elements = Vec::new();
        for element in self.x.iter() {
            if ! Vect::exists(element, values) { without_elements.push(element) }
        }

        return without_elements;
    }

    /// it should return &'a Vec<T>?
    pub fn union(&'a self, adds: &Vec<T>) -> Vec<T> {
        self.x.add(adds)
    }

    pub fn intersection(&'a self, intersec: &Vec<T>) -> Vec<&'a T> {
        let mut intersected = Vec::new();
        for element in self.x.iter() {
            if Vect::exists(element, intersec) { intersected.push(element) }
        }

        return intersected;
    }

    pub fn uniq(&'a self) -> Vec<&'a T> {
        let mut uniq = Vec::new();
        for element in self.x.iter() {
            if ! Vect::exists(&element, &uniq) { uniq.push(element) }
        }

        return uniq;
    }
}

