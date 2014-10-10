extern crate __;

use std::collections::HashMap;

#[test]
fn test_vec_first() {
    assert_eq!(1i, *__::vec::Vect::new(vec!(1i, 2, 3)).first().unwrap());
    assert_eq!("aa", *__::vec::Vect::new(vec!("aa", "bb")).first().unwrap());
}

#[test]
fn test_vec_without() {
    let vec_int = vec!(1i, 2i, 2i);
    for x in __::vec::Vect::new(vec_int).without(&vec!(1i)).iter() {
        assert_eq!(2i, *x);
    }

    let vec_str = vec!("aa", "bb", "bb", "cc");
    for x in __::vec::Vect::new(vec_str).without(&vec!("bb", "cc")).iter() {
        assert_eq!("aa", *x);
    }
}

#[test]
fn test_vec_intersection() {
    let intersected = __::vec::Vect::new(vec!(1i, 2, 3)).intersection(&vec!(2i, 3, 4));
    assert_eq!(vec!(2i, 3), intersected);
}

#[test]
fn test_vec_uniq() {
    let uniqed = __::vec::Vect::new(vec!(0u, 1, 1, 2, 2, 3)).uniq();
    assert_eq!(vec!(0u, 1, 2, 3), uniqed);
}

#[test]
fn test_hash_pairs() {
    let mut sample = HashMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);
    let pairs = __::hashmap::Hashing::new(sample).pairs();

    assert_eq!((1i, 1u), pairs[0]);
    assert_eq!((2i, 2u), pairs[1]);
}

#[test]
fn test_hash_invert() {
    let mut sample = HashMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);

    let inverted = __::hashmap::Hashing::new(sample).invert();
    assert!(inverted.contains_key(&1u));
    assert!(inverted.contains_key(&2u));
}

#[test]
fn test_hash_pick() {
    let mut sample = HashMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);

    let picked = __::hashmap::Hashing::new(sample).pick(vec!(1i));
    for key in picked.keys() {
        assert_eq!(1u, picked.get_copy(key));
    }
}

#[allow(unused_variable)]
fn sample_filter(x: &int, y: &uint) -> bool {
    if *x == 1 { return true; }
    return false;
}

#[test]
fn test_hash_pick_by_filter() {
    let mut sample = HashMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);

    let picked = __::hashmap::Hashing::new(sample).pick_by_filter(sample_filter);
    for key in picked.keys() {
        assert_eq!(1u, picked.get_copy(key));
    }
}

#[test]
fn test_hash_omit() {
    let mut sample = HashMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);

    let omitted = __::hashmap::Hashing::new(sample).omit(vec!(1i));
    for key in omitted.keys() {
        assert_eq!(2u, omitted.get_copy(key));
    }
}

#[test]
fn test_hash_omit_by_filter() {
    let mut sample = HashMap::new();
    sample.insert(1i, 1u);
    sample.insert(2i, 2u);

    let omitted = __::hashmap::Hashing::new(sample).omit_by_filter(sample_filter);
    for key in omitted.keys() {
        assert_eq!(2u, omitted.get_copy(key));
    }
}

#[test]
fn test_hash_defaults() {
    let mut origin = HashMap::new();
    origin.insert(1i, 1u);
    origin.insert(2i, 2u);

    let mut appends = HashMap::new();
    appends.insert(1i, 10000u);
    appends.insert(3i, 3u);

    let defaults = __::hashmap::Hashing::new(origin).defaults(appends);
    for x in vec!(1i, 2, 3).iter() {
        assert!(defaults.contains_key(x));
        assert_eq!(*x as uint, defaults.get_copy(x));
    }
}
