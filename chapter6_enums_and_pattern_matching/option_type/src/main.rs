fn main() {
    let some_value = Some(8);
    println!("{some_value:?}");
    println!("some_value is: {}", some_value.unwrap());

    let no_value: Option<i32> = None;
    println!("{no_value:?}");

    // cannot perform some_value + 4 by default (Option<i32> + 4) because they are different types.
    // the unwrap method returns the value contained in some
    let sum = some_value.unwrap() + 4;
    println!("{sum}");
}
