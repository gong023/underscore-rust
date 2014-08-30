extern crate underscore;

#[test]
fn test_vec_first() {
    let __ = underscore::Vect;
    let vec_int = vec!(1i, 2i, 3i);
    let first_int = __.first(&vec_int);
    let vec_str = vec!("aa", "bb", "cc");
    let first_str = __.first(&vec_str);

    assert_eq!(1i, *first_int);
    assert_eq!("aa", *first_str);
}

#[test]
fn test_vec_last() {
    let __ = underscore::Vect;
    let vec_int = vec!(1i, 2i, 3i);
    let last_int = __.last(&vec_int);
    let vec_str = vec!("aa", "bb", "cc");
    let last_str = __.last(&vec_str);

    assert_eq!(3i, *last_int);
    assert_eq!("cc", *last_str);
}

#[test]
fn test_vec_initial() {
    let __ = underscore::Vect;

    let vec_int = vec!(1i, 1i, 2i);
    let initial_vec = __.initial(&vec_int, 2u);
    for x in initial_vec.iter() {
        assert_eq!(1i, **x);
    }

    let vec_str = vec!("aa", "aa", "bb");
    let initial_vec_str = __.initial(&vec_str, 2u);
    for x in initial_vec_str.iter() {
        assert_eq!("aa", **x);
    }
}

#[test]
fn test_vec_rest() {
    let __ = underscore::Vect;

    let vec_int = vec!(1i, 2i, 3i, 3i);
    let rest_vec = __.rest(&vec_int, 2u);
    for x in rest_vec.iter() {
        assert_eq!(3i, **x);
    }

    let vec_str = vec!("aa", "bb", "cc", "cc");
    let rest_str = __.rest(&vec_str, 2u);
    for x in rest_str.iter() {
        assert_eq!("cc", **x);
    }
}
