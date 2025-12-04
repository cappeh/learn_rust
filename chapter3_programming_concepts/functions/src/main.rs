// main is the entry point to most rust programs (Binary project)
fn main() {
    println!("Hello, world!");

    another_function(); // calls another_function
    another_function_with_paramater(5);
    print_labeled_measurement(5, 'h'); // char should be in '' (single quotes)

    let _y = 6; // this is a statement. adding semi colon to an expression would make it a
                // statement

    let _x = {
        let y = 3;
        y + 1
    }; // this is an expression within a statement

    let five = five(); // using the return value of the function the same as let five = 5;
    println!("the value of five is: {five}");

    let six = plus_one(five); // takes an i32 as an argument and returns an i32
    println!("the value of five is: {six}");
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

fn five() -> i32 {
    5 // this is an expression that returns the value 5 (i32)
}

fn plus_one(x: i32) -> i32 {
    x + 1 // adds one to the value of x passed as an argument to the function call
}
