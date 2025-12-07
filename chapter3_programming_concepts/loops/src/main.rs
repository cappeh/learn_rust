fn main() {
    let mut number = 3;
    //
    // while number != 0 {
    //     println!("{number}");
    //     number -= 1;
    // }
    //
    // println!("LIFT OFF!!!");

    loop {
        if number == 0 {
            break;
        }
        println!("{number}");
        number -= 1;
    }
    println!("LIFT OFF!!!");
}

