use rust_test_app; // use our crate (this app just like if it was a published crate)

mod common; // import our modules in our common tests folder

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, rust_test_app::adder(2, 2));
}

#[test]
fn it_adds_two_calling_two() {
    let first = rust_test_app::adder(1, 1);
    let second = rust_test_app::adder(2, 2);
    assert_eq!(
        (first + second) * 2,
        rust_test_app::adder(
            rust_test_app::adder(first, second),
            rust_test_app::adder(first, second)
        )
    );
}
