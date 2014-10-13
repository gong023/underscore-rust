extern crate __;

use __::vec::VecUnderscore;
use std::collections::HashMap;

#[test]
fn test_vec_first() {
    let v = vec!(1i, 2, 3);
    assert_eq!(1i, *v.first().unwrap());
}

#[test]
fn test_vec_without() {
    let v = vec!(1i, 2, 2);
    for x in v.without(&vec!(1i)).iter() {
        assert_eq!(2i, *x);
    }
}

#[test]
fn test_vec_intersection() {
    let v = vec!(1i, 2, 3);
    assert_eq!(vec!(2i, 3), v.intersection(&vec!(2i, 3)));
}

#[test]
fn test_vec_uniq() {
    let v = vec!(0u, 1, 1, 2, 2, 3);
    assert_eq!(vec!(0u, 1, 2, 3), v.uniq());
}

#[test]
fn test_vec_index_of() {
    assert_eq!(2u, vec!(3i, 2, 1).index_of(&1i).unwrap());
    assert_eq!(None, vec!(3i, 2, 1).index_of(&4i));
}

#[test]
fn test_vec_last_index_of() {
    assert_eq!(3u, vec!(1i, 2, 3, 1).last_index_of(&1i).unwrap());
    assert_eq!(0u, vec!(1i, 2, 3).last_index_of(&1i).unwrap());
    assert_eq!(None, vec!(3i, 2, 1).last_index_of(&4i));
}

#[test]
fn test_vec_object() {
    let obj = vec!(0u, 1, 2, 3).object(vec!(0i, 1, 2, 3));

    for key in range(0u, obj.len()) {
        match obj.find(&key) {
            Some(value) => assert_eq!(*value as uint, key),
            None => fail!("{} not found at test_vec_object", key),
        }
    }
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
