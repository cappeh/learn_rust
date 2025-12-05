fn main() {
    // in the following code Ctrl-C will need to be used to interrupt the program
    // loop {
    //     println!("again");
    // }
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");
}
