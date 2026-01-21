use std::{thread, time::Duration};

fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let closure = expensive_closure(5);
    println!("{closure}");

    let add_one_v2 = |x: u32| -> u32 {x + 1};
    let add_one_v3 = |x| {x + 1};
    let add_one_v4 = |x| x + 1;

    println!("{}", add_one_v1(9));
    println!("{}", add_one_v2(5));
    println!("{}", add_one_v3(8));
    println!("{}", add_one_v4(4));

    // ==================================================================

    // Borrowing immutably in a closure
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // ==================================================================

    // Borrowing Mutably
    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {list2:?}");

    let mut borrows_mutably = || list2.push(7);

    borrows_mutably();
    println!("After calling closure: {list2:?}");

    // ==================================================================

    // moving a value/ taking ownership with threads
    let list3 = vec![1, 2, 3];
    println!("Before defining closurel: {list3:?}");

    thread::spawn(move || println!("From thread: {list3:?}"))
        .join()
        .unwrap();

    // ==================================================================
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}
