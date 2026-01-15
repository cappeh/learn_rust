fn add_one_v1(x: i32) -> i32 {
    x + 1
}

fn main() {
    let add_one_v2 = |x: i32| -> i32 {
        x + 1
    };

    let add_one_v3 = |x: i32| x + 1;

    let add_one_v4 = |x: i32| x + 1;


    println!("{}", add_one_v2(4));
}
