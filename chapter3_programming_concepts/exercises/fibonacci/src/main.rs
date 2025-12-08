fn fibonacci(nth: u64) -> u64 {
    if nth < 2 {
        return nth;
    }
    let mut fib = (0, 1);

    for _ in 1..=nth {
        fib = (fib.1, fib.0 + fib.1);
    }
    fib.0
}

fn main() {
    println!("fibonacci({}) is: {}", 10, fibonacci(10));
}
