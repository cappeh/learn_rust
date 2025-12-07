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

    let mut count = 0;

    'counting_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");
}

