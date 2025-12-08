const TARGET_NUMBER: i32 = 100;

fn main() {
    for num in 1..=TARGET_NUMBER {
        if num % 15 == 0 {
            println!("FizzBuzz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else if num % 3 == 0 {
           println!("Fizz");
        } else {
            println!("{num}");
        }
    }
}
