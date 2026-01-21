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
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}
