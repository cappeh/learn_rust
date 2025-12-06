fn main() {
    // in the following code Ctrl-C will need to be used to interrupt the program
    // loop {
    //     println!("again");
    // }
    let mut counter = 0; // declare and initialize counter

    let result = loop { // result will hold the value returned from the loop
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}"); // will print 20
}
