fn factorial(number: u32) -> u32 {
    let mut result = 1;
    for i in 1..=number {
        result *= i;
    }
    result
}

fn main() {
    let num = 5;
    println!("the factorial for {num} is: {}", factorial(num))
}
