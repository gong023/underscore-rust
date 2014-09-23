pub struct Vect;

impl Vect {
    pub fn first<'a, T>(self, v: &'a Vec<T>) -> &'a T {
        &v[0]
    }

    pub fn last<'a, T>(self, v: &'a Vec<T>) -> &'a T {
        &v[v.len() - 1]
    }

    pub fn initial<'a, T>(self, v: &'a Vec<T>, n: uint) -> Vec<&'a T> {
        let mut initialized = Vec::new();
        for i in range(0u, n) {
            initialized.push(&v[i])
        }

        return initialized;
    }

    pub fn rest<'a, T>(self, v: &'a Vec<T>, n: uint) -> Vec<&'a T> {
        let mut rest = Vec::new();
        for i in range(0u, v.len()) {
            if i < n { continue; }
            rest.push(&v[i])
        }

        return rest;
    }

    pub fn exists<T: PartialEq>(self, x: &T, v: &Vec<T>) -> bool {
        for element in v.iter() {
            if element.eq(x) { return true; }
        }

        return false;
    }

    pub fn without<'a, T: PartialEq>(self, v: &'a Vec<T>, values: &Vec<T>) -> Vec<&'a T> {
        let mut without_elements = Vec::new();
        for element in v.iter() {
            if ! self.exists(element, values) {
                without_elements.push(element)
            }
        }

        return without_elements;
    }

    pub fn union<T: Clone>(self, origin: &Vec<T>, adds: &Vec<T>) -> Vec<T> {
        origin.add(adds)
    }

    pub fn intersection<'a, T: PartialEq>(self, v1: &'a Vec<T>, v2: &Vec<T>) -> Vec<&'a T> {
        let mut intersected = Vec::new();
        for element in v1.iter() {
            if self.exists(element, v2) { intersected.push(element) }
        }

        return intersected;
    }

    pub fn uniq<T: PartialEq>(self, v: Vec<T>) -> Vec<T> {
        let mut uniq = Vec::new();
        for element in v.move_iter() {
            if ! self.exists(&element, &uniq)  { uniq.push(element) }
        }

        return uniq;
    }
}
