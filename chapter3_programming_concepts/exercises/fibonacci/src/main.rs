fn fibonacci(nths: i32) {
    let mut a = 0;
    let mut b = 1;

    println!("{a}");
    for _ in 1..=nths {
        let next = a + b;
        a = b;
        b = next;
        println!("{a}");
    }
}

fn main() {
    fibonacci(5);
}
