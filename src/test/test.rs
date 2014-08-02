extern crate underscore;

#[test]
fn test_hello() {
    let __ = underscore::U;
    assert_eq!(1i, __.hello());
}
