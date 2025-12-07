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

    // removing 50 and having 4 elements will cause panic: index out of bounds if the while loop
    // index is not updated to 4
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // here the index would need to be compared against the length of the array to check whether
    // the index is still in bounds
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // more effiecient because the index does not need to be compared against the length of the
    // array and increases the safety of the code and eleminated bugs
    // because an element will not be missed if the index is incorrect or exceeding the lenght of
    // the array
    for element in a {
        println!("the value is: {}", element);
    }

    // (1..=4) means 1 to 4 inclusive
    // the below is 1 to 3. the last number is excluded
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFT OFF")
}

