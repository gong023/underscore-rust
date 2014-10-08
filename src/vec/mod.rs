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

    pub fn first(&'a self) -> Option<&'a T> {
        if *&self.x.as_slice().is_empty() {
            None
        } else {
            Some(&self.x[0])
        }
    }

    pub fn without(&'a self, values: &Vec<T>) -> Vec<&'a T> {
        let mut without_elements = Vec::new();
        for element in self.x.iter() {
            if ! Vect::exists(element, values) { without_elements.push(element) }
        }

        return without_elements;
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

