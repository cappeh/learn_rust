use integration_tests_adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}

#[test]
fn it_adds_three() {
    let result = add_two(3);
    assert_eq!(result, 5);
}
