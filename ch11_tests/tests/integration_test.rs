use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

// Unit tests exercise different parts of a library separately and can test private implementation details.
// Integration tests check that many parts of the library work together correctly,
// and they use the library’s public API to test the code in the same way external code will use it.
