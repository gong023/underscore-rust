extern crate __;

#[test]
fn test_vec_first() {
    assert_eq!(1i, *__::Vect::new(vec!(1i, 2, 3)).first());
    assert_eq!("aa", *__::Vect::new(vec!("aa", "bb")).first())
}

#[test]
fn test_vec_last() {
    assert_eq!(3i, *__::Vect::new(vec!(1i, 2, 3)).last());
    assert_eq!("bb", *__::Vect::new(vec!("aa", "bb")).last())
}

#[test]
fn test_vec_initial() {
    let vec_int = vec!(1i, 1i, 2i);
    for x in __::Vect::new(vec_int).initial(2u).iter() {
        assert_eq!(1i, **x)
    }

    let vec_str = vec!("aa", "aa", "bb");
    assert_eq!(2u, __::Vect::new(vec_str).initial(2u).len());
}

#[test]
fn test_vec_rest() {
    let vec_int = vec!(1i, 2i, 3i, 3i);
    for x in __::Vect::new(vec_int).rest(2u).iter() {
        assert_eq!(3i, **x);
    }

    let vec_str = vec!("aa", "bb", "cc", "cc");
    assert_eq!(2u, __::Vect::new(vec_str).rest(2u).len());
}

#[test]
fn test_vec_exists() {
    assert!(__::Vect::exists(&1i, &vec!(1i, 2, 3)));
}

#[test]
fn test_vec_without() {
    let vec_int = vec!(1i, 2i, 2i);
    for x in __::Vect::new(vec_int).without(&vec!(1i)).iter() {
        assert_eq!(2i, **x);
    }

    let vec_str = vec!("aa", "bb", "bb", "cc");
    for x in __::Vect::new(vec_str).without(&vec!("bb", "cc")).iter() {
        assert_eq!("aa", **x);
    }
}

#[test]
fn test_vec_union() {
    let union_int_vec = __::Vect::new(vec!(1i, 2, 3)).union(&vec!(4i, 5i, 6i));
    assert_eq!(vec!(1i, 2, 3, 4, 5, 6), union_int_vec);
}

#[test]
fn test_vec_intersection() {
    let intersect_int_vec = __::Vect::new(vec!(1i, 2, 3));
    assert_eq!(1u, intersect_int_vec.intersection(&vec!(2i, 4)).len());
    assert_eq!(2i, *intersect_int_vec.intersection(&vec!(2i, 4))[0]);
}

#[test]
fn test_vec_uniq() {
    let vec_int = __::Vect::new(vec!(0u, 1, 1, 2, 2, 3));
    assert_eq!(0u, *vec_int.uniq()[0]);
    assert_eq!(1u, *vec_int.uniq()[1]);
    assert_eq!(4u, vec_int.uniq().len());
}
