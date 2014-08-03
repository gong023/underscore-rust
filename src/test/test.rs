extern crate underscore;

#[test]
fn test_hello() {
    let __ = underscore::U;
    assert_eq!(1i, __.hello());
}

#[test]
fn test_arr_first() {
    let __ = underscore::U;
    assert_eq!(1i, __.first([1i, ..3]));
//    assert_eq!("hoge", __.first(["hoge", "fuga"]));
}
