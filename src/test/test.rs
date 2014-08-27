extern crate underscore;

#[test]
fn test_hello() {
    let __ = underscore::U;
    assert_eq!(1i, __.hello());
}

#[test]
fn test_vec_first() {
    let __ = underscore::U;
    let vec_int = vec!(1i, 2i, 3i);
    let first_int = __.first(&vec_int);
    let vec_str = vec!("aa", "bb", "cc");
    let first_str = __.first(&vec_str);

    assert_eq!(1i, *first_int);
    assert_eq!("aa", *first_str);
}

#[test]
fn test_vec_last() {
    let __ = underscore::U;
    let vec_int = vec!(1i, 2i, 3i);
    let last_int = __.last(&vec_int);
    let vec_str = vec!("aa", "bb", "cc");
    let last_str = __.last(&vec_str);

    assert_eq!(3i, *last_int);
    assert_eq!("cc", *last_str);
}
