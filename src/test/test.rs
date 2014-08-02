extern crate underscore;

#[test]
fn test_hello() {
    let ret = U::hello();
    assert_eq!(1i, ret);
}
