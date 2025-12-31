fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("the largest number is: {result}");
    // let mut largest = &number_list[0];
    //
    // for num in &number_list {
    //     if num > largest {
    //         largest = num;
    //     }
    // }
    // println!("the largest number is: {largest}");
    //
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("the largest number is: {result}");
    // let mut largest = &number_list[0];
    //
    // for num in &number_list {
    //     if num > largest {
    //         largest = num;
    //     }
    // }
    // println!("the largest number is {largest}");
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
