extern crate __;

use std::collections::HashMap;

#[test]
fn test_vec_first() {
    assert_eq!(1i, *__::vec::Vect::new(vec!(1i, 2, 3)).first());
    assert_eq!("aa", *__::vec::Vect::new(vec!("aa", "bb")).first())
}

#[test]
fn test_vec_exists() {
    assert!(__::vec::Vect::exists(&1i, &vec!(1i, 2, 3)));
}

#[test]
fn test_vec_without() {
    let vec_int = vec!(1i, 2i, 2i);
    for x in __::vec::Vect::new(vec_int).without(&vec!(1i)).iter() {
        assert_eq!(2i, **x);
    }

    let vec_str = vec!("aa", "bb", "bb", "cc");
    for x in __::vec::Vect::new(vec_str).without(&vec!("bb", "cc")).iter() {
        assert_eq!("aa", **x);
    }
}

#[test]
fn test_vec_intersection() {
    let intersect_int_vec = __::vec::Vect::new(vec!(1i, 2, 3));
    assert_eq!(1u, intersect_int_vec.intersection(&vec!(2i, 4)).len());
    assert_eq!(2i, *intersect_int_vec.intersection(&vec!(2i, 4))[0]);
}

#[test]
fn test_vec_uniq() {
    let vec_int = __::vec::Vect::new(vec!(0u, 1, 1, 2, 2, 3));
    assert_eq!(0u, *vec_int.uniq()[0]);
    assert_eq!(1u, *vec_int.uniq()[1]);
    assert_eq!(4u, vec_int.uniq().len());
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
