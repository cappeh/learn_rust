// main is the entry point to most rust programs (Binary project)
fn main() {
    println!("Hello, world!");

    another_function(); // calls another_function
    another_function_with_paramater(5);
    print_labeled_measurement(5, 'h'); // char should be in '' (single quotes)
}

// could be placed before the main function. you can define functions anywhere as long as they are
// defined in a scope that can be seen by the caller
fn another_function() {
    println!("another function");
}

// the function signature has one parameter (x) of type i32
fn another_function_with_paramater(x: i32) {
    println!("the value of x is: {x}"); // the value of x is used here
}

// multiple params in the signature are seperated by commas
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}
